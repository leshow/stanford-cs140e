use console::{kprint, kprintln, CONSOLE};
use stack_vec::StackVec;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

const WELCOME: &'static str = r#"
                                                         c=====e
                                                            H
   ____________                                         _,,_H__
  (__((__((___()                                       //|     |
 (__((__((___()()_____________________________________// |ACME |
(__((__((___()()()------------------------------------'  |_____|
"#;

const DEL: u8 = 127;
const BKSP: u8 = 8;
const BELL: u8 = 7;
/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    kprintln!("{}", WELCOME);
    let mut hist_buf = [0u8; 512];
    let mut history = StackVec::new(&mut hist_buf);

    loop {
        kprint!("{}", prefix);
        let mut cmd_buf = [0u8; 512];
        let mut cmd = StackVec::new(&mut cmd_buf);
        loop {
            let byte = CONSOLE.lock().read_byte();
            match byte {
                b'\r' | b'\n' => {}
                BKSP => {
                    if cmd.pop().is_none() {
                        CONSOLE.lock().write_byte(BELL);
                    }
                }
                _ => {
                    cmd.push(byte);
                }
            }
        }
    }
}
