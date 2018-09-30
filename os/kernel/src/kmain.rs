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

mod cmd;
pub mod console;
pub mod lang_items;
pub mod mutex;
pub mod shell;

use pi::{
    gpio::{Gpio, Output},
    timer,
    uart::MiniUart,
};
use std::{
    fmt::Write,
    io::{Read, Write as ioWrite},
};

#[no_mangle]
pub extern "C" fn kmain() {
    shell::shell("> ")
    // timer::spin_sleep_ms(2000);
    // let mut uart = MiniUart::new();
    // uart.write_str("Hello, world!\n\n");
    // uart.set_read_timeout(5000);
    // loop {
    //     uart.write_str("> ");
    //     let mut buf = [0u8; 16];
    //     match uart.read(&mut buf) {
    //         Err(_) => {
    //             uart.write_str("\nTimeout\n");
    //             continue;
    //         }
    //         Ok(bytes) => {
    //             std::fmt::Write::write_fmt(&mut uart, format_args!("\nGOT({}): ", bytes));
    //             uart.write(&buf[0..bytes]);
    //             uart.write_str("\n");
    //         }
    //     }
    //     timer::spin_sleep_ms(25);
    // }
}
