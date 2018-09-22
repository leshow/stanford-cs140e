#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(use_nested_groups)]

extern crate pi;
extern crate stack_vec;

pub mod console;
pub mod lang_items;
pub mod mutex;
pub mod shell;

use pi::{
    gpio::{Gpio, Output},
    timer,
    uart::MiniUart,
};

use std::fmt::Write;

#[no_mangle]
pub extern "C" fn kmain() {
    let mut uart = MiniUart::new();
    loop {
        let byte = uart.read_byte();
        uart.write_byte(byte);
        uart.write_str("<-");
    }
    // let mut p5: Gpio<Output> = Gpio::new(5).into_output();
    // let mut p6: Gpio<Output> = Gpio::new(6).into_output();
    // let mut p13: Gpio<Output> = Gpio::new(13).into_output();
    // let mut p19: Gpio<Output> = Gpio::new(19).into_output();
    // let mut p26: Gpio<Output> = Gpio::new(26).into_output();
    // loop {
    //     p5.set();
    //     timer::spin_sleep_ms(500);
    //     p5.clear();
    //     // 6
    //     p6.set();
    //     timer::spin_sleep_ms(500);
    //     p6.clear();
    //     // 13
    //     p13.set();
    //     timer::spin_sleep_ms(500);
    //     p13.clear();
    //     // 19
    //     p19.set();
    //     timer::spin_sleep_ms(500);
    //     p19.clear();
    //     // 26
    //     p26.set();
    //     timer::spin_sleep_ms(500);
    //     p26.clear();
    // }
}
