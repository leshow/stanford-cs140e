#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;
pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL0: *mut u32 = GPIO_BASE as *mut u32;
const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

const GPIO_MASK: u32 = 0b111;
const GPIO_OUT: u32 = 0b001;
const GPIO_IN: u32 = 0b000;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 600) {
        unsafe {
            asm!("nop" :::: "volatile");
        }
    }
}

pub struct GPIO {
    pin: usize,
    mode: Mode,
}

enum Mode {
    In,
    Out,
}

impl Mode {
    pub fn get_flag(&self) -> u32 {
        match *self {
            Mode::Out => GPIO_OUT,
            Mode::In => GPIO_IN,
        }
    }
}

impl GPIO {
    pub fn new(pin: usize, mode: Mode) -> GPIO {
        assert!(pin <= 53, "Pin out of range");
        let shift = (pin % 10) * 3;
        let offset = pin / 10;
        let gpio_pin: *mut u32 = GPIO_FSEL0.offset(offset as isize);
        unsafe {
            let gpio_cur: u32 = gpio_pin.read_volatile();
            let val = (gpio_cur & !(GPIO_MASK << shift)) | (mode.get_flag() << shift);
            gpio_pin.write_volatile(val);
            return GPIO { pin, mode };
        }
    }

    pub fn set(&self) {
        let offset = self.pin / 32;
        unsafe {
            GPIO_SET0.add(offset).write_volatile(1 << self.pin);
        }
    }

    pub fn clear(&self) {
        let offset = self.pin / 32;
        unsafe {
            GPIO_CLR0.add(offset).write_volatile(1 << self.pin);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // STEP 1: Set GPIO Pin 16 as output.
    let pin = GPIO::new(16, Mode::Out);
    // STEP 2: Continuously set and clear GPIO 16.
    loop {
        pin.set();
        spin_sleep_ms(1000);
        pin.clear();
        spin_sleep_ms(1000);
    }
}
