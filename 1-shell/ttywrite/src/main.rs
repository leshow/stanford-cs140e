extern crate pbr;
extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use]
extern crate structopt_derive;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    sync::RwLock,
    time::Duration,
};

use pbr::{ProgressBar, Units};
use serial::core::{BaudRate, CharSize, FlowControl, SerialDevice, SerialPortSettings, StopBits};
use structopt::StructOpt;

use xmodem::{Progress, Xmodem};

mod input;
mod parsers;
use parsers::{parse_baud_rate, parse_flow_control, parse_stop_bits, parse_width};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(
        short = "i",
        help = "Input file (defaults to stdin if not set)",
        parse(from_os_str)
    )]
    input: Option<PathBuf>,

    #[structopt(
        short = "b",
        long = "baud",
        parse(try_from_str = "parse_baud_rate"),
        help = "Set baud rate",
        default_value = "115200"
    )]
    baud_rate: BaudRate,

    #[structopt(
        short = "t",
        long = "timeout",
        parse(try_from_str),
        help = "Set timeout in seconds",
        default_value = "10"
    )]
    timeout: u64,

    #[structopt(
        short = "w",
        long = "width",
        parse(try_from_str = "parse_width"),
        help = "Set data character width in bits",
        default_value = "8"
    )]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(
        short = "f",
        long = "flow-control",
        parse(try_from_str = "parse_flow_control"),
        help = "Enable flow control ('hardware' or 'software')",
        default_value = "none"
    )]
    flow_control: FlowControl,

    #[structopt(
        short = "s",
        long = "stop-bits",
        parse(try_from_str = "parse_stop_bits"),
        help = "Set number of stop bits",
        default_value = "1"
    )]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let mut serial = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    let mut settings = serial.read_settings()?;
    settings.set_baud_rate(opt.baud_rate)?;
    settings.set_char_size(opt.char_width);
    settings.set_stop_bits(opt.stop_bits);
    settings.set_flow_control(opt.flow_control);

    serial.write_settings(&settings)?;
    serial.set_timeout(Duration::from_secs(opt.timeout))?;

    let mut reader: Box<dyn BufRead> = match opt.input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(input) => Box::new(BufReader::new(File::open(input).expect("Error with path"))),
    };
    let buf_len = reader.buffer().len();
    let mut pb = ProgressBar::new(buf_len);
    pb.set_units(Units::Bytes);

    if opt.raw {
        let total_bytes = io::copy(&mut reader, &mut serial)?;
        println!("Wrote {} bytes", total_bytes);
    } else {
        let total_bytes =
            Xmodem::transmit_with_progress(reader, serial, |progress| match progress {
                Progress::Started => {
                    println!("Starting transmission...");
                }
                Progress::Waiting => {
                    pb.tick();
                }
                Progress::Packet(pkt) => {
                    // println!("wrote: {} bytes to input", pkt);
                    pb.set((pkt % 255) as u64);
                }
            })?;
        println!("Wrote {} bytes", total_bytes);
        pb.finish();
    }
    Ok(())
}
