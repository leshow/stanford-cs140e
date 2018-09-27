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
};

#[no_mangle]
pub extern "C" fn kmain() {
    shell::shell("> ");
}
