use console::{kprint, kprintln};

pub(crate) fn echo(args: &[&str]) -> bool {
    for &arg in args {
        kprint!("{} ", arg);
    }
    true
}
