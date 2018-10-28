#![allow(dead_code, unused_imports)]
#![feature(decl_macro, conservative_impl_trait)]
#![allow(safe_packed_borrows)]

#[cfg(not(target_endian = "little"))]
compile_error!("only little endian platforms supported");

mod mbr;
#[cfg(test)]
mod tests;
mod util;

pub mod traits;
pub mod vfat;

pub use mbr::*;
