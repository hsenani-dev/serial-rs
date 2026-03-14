use clap::Parser;
use serialport::{self, SerialPort, SerialPortInfo};
use std::io::{Read, Write};
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(author = "Henry Senanian", version, about)]
/// Application configuration
struct Args {
    /// path of serial device
    #[arg()]
    dev: String,

    /// baudrate to be used. If omitted, uses 921600
    #[arg(short, long)]
    baudrate: Option<u32>,

    /// path of binary to be flashed
    #[arg()]
    bin: String,
}

fn main() {
    let args = Args::parse();

    let ports = serialport::available_ports().expect("No ports found");
    // let port
    for p in &ports {
        println!("{}", p.port_name);
        if args.dev == p.port_name {
            println!("valid arg");
        }
    }

    let baudrate = args.baudrate.unwrap_or(921600);
    println!("Using {} with {} baud.", args.dev, baudrate);

    let mut port = serialport::new(args.dev, baudrate)
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
