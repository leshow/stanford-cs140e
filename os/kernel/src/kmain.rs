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
    // let mut uart = MiniUart::new();
    // loop {
    //     let byte = uart.read_byte();
    //     uart.write_byte(byte);
    //     uart.write_str("<-");
    // }
}
