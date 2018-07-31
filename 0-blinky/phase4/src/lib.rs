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
    mode: Mode
}

enum Mode {
    In, Out
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
        if pin > 53 {
            assert!("Unable to set a pin that high, pin 52 is the max");
        }
        let shift = (pin % 10) * 3;
        let offset = pin  10;
        let gpio_pin: *mut u32 = GPIO_FSEL0.offset(offset);
        let gpio_cur: u32 = gpio_pin.read_volatile();
        unsafe {
            let val = gpio_cur & !(GPIO_MASK << shift);
            gpio_pin.write_volatile(val | (mode.get_flag() << shift));

            return GPIO {
                pin,
                mode
            }
        }
    }
}




#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // STEP 1: Set GPIO Pin 16 as output.
    let pin = GPIO::new(16, Mode::Out);
    // STEP 2: Continuously set and clear GPIO 16.
}
