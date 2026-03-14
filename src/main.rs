use clap::Parser;
use serialport::{self, SerialPort, SerialPortInfo};
use std::io::{Read, Write};
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(author = "Henry Senanian", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    dev: Option<String>,

    #[arg()]
    baudrate: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }

    let ports = serialport::available_ports().expect("No ports found");
    // let port
    for p in &ports {
        println!("{}", p.port_name);
        if args.dev.clone().unwrap_or_default() == p.port_name {
            println!("valid arg");
        }
    }

    let mut port = serialport::new(args.dev.clone().unwrap_or_default(), 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let mut serial_buf: Vec<u8> = vec![0; 32];
    match port.read(serial_buf.as_mut_slice()) {
        Ok(t) => println!("Read {} bytes: {:?}", t, &serial_buf[..t]),
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    };

    // println!(
    //     "Updating using {} with baudrate {}...",
    //     args.dev.unwrap(),
    //     args.baudrate.unwrap()
    // );
}
