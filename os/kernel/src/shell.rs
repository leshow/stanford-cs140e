use cmd;
use console::{kprint, kprintln, CONSOLE};
use stack_vec::StackVec;
use std::str::from_utf8;
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

    fn execute(&self) -> bool {
        match self.path() {
            "echo" => cmd::echo(&self.args[1..]),
            _ => false,
        }
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

const WELCOME: &str = r#"
Uh-oh...
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
const LF: u8 = 10;
const CR: u8 = 13;

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    kprintln!("{}", WELCOME);
    loop {
        kprint!("{}", prefix);
        match readline() {
            Err(Error::TooManyArgs) => {
                kprintln!("Slow down cowboy, too many arguments");
            }
            Err(Error::Empty) => {
                kprintln!("");
            }
            Ok(run) => if !run.execute() {
                kprintln!("unknown command: {}", run.path());
            },
        }
    }
}

fn readline<'a>() -> Result<Command<'a>, Error> {
    let console = CONSOLE.lock();
    let mut buf = [0u8; 512];
    let mut stack = StackVec::new(&mut buf);

    loop {
        let byte = console.read_byte();
        match byte {
            BKSP | DEL => {
                if stack.pop().is_none() {
                    console.write_byte(BELL);
                } else {
                    console.write_byte(BKSP);
                    console.write_byte(b' ');
                    console.write_byte(BKSP);
                }
            }
            _ => {
                if stack.push(byte).is_err() {
                    console.write_byte(BELL);
                } else {
                    console.write_byte(byte);
                }
            }
            CR | LF => {
                let mut cmd_buf: [&str; 64] = [""; 64];
                let cmd = from_utf8(stack.as_slice()).unwrap_or_default();
                return Command::parse(cmd, &mut cmd_buf);
            }
        }
    }
}
