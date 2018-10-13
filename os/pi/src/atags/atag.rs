use core::{slice, str};
// use std::ffi::CStr;
// use std::os::raw::c_char;
use atags::raw;

pub use atags::raw::{Core, Mem};

/// An ATAG.
#[derive(Debug, Copy, Clone)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None,
}

impl Atag {
    /// Returns `Some` if this is a `Core` ATAG. Otherwise returns `None`.
    pub fn core(self) -> Option<Core> {
        match self {
            Atag::Core(core) => Some(core),
            _ => None,
        }
    }

    /// Returns `Some` if this is a `Mem` ATAG. Otherwise returns `None`.
    pub fn mem(self) -> Option<Mem> {
        match self {
            Atag::Mem(mem) => Some(mem),
            _ => None,
        }
    }

    /// Returns `Some` with the command line string if this is a `Cmd` ATAG.
    /// Otherwise returns `None`.
    pub fn cmd(self) -> Option<&'static str> {
        match self {
            Atag::Cmd(cmd) => Some(cmd),
            _ => None,
        }
    }
}

impl<'a> From<&'a raw::Atag> for Atag {
    fn from(atag: &raw::Atag) -> Atag {
        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => Atag::Core(core),
                (raw::Atag::MEM, &raw::Kind { mem }) => Atag::Mem(mem),
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => {
                    let ptr = &(*cmd).cmd as *const u8;
                    // no access to std::ffi here it looks like, if we did:
                    // let slice = Cstr::from_ptr(ptr as *const c_char); // gets null-terminated string to avoid while loop below
                    // Atag::Cmd(str::from_utf8_unchecked(slice.to_bytes()))
                    let mut len = 0;
                    while *ptr.add(len) != 0 {
                        len += 1;
                    }
                    Atag::Cmd(str::from_utf8_unchecked(slice::from_raw_parts(ptr, len)))
                }
                (raw::Atag::NONE, _) => Atag::None,
                (id, _) => Atag::Unknown(id),
            }
        }
    }
}
