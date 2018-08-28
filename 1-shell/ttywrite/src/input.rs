// This was my first attempt to polymorphically get the input src from
// either File or Stdin. It better encapsulates things but
// for such a small bit of code, I'll just use Box<dyn BufRead>
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[allow(dead_code)]
struct In<'a> {
    src: Box<dyn BufRead + 'a>,
}

#[allow(dead_code)]
impl<'a> In<'a> {
    fn stdin(stdin: &'a io::Stdin) -> In<'a> {
        In {
            src: Box::new(stdin.lock()),
        }
    }
    fn file<P: AsRef<Path>>(path: P) -> io::Result<In<'a>> {
        File::open(path.as_ref()).map(|file| In {
            src: Box::new(io::BufReader::new(file)),
        })
    }
}

impl<'a> io::Read for In<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.src.read(buf)
    }
}

impl<'a> io::BufRead for In<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.src.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.src.consume(amt);
    }
}
