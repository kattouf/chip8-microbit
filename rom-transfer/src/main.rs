#![feature(error_in_core)]

use core::error::Error;
use scp::encoder::Encoder;
use serialport::SerialPort;
use std::{env, fs, process, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match run(config) {
        // WORKAROUND: sleep process to keep the port alive
        Ok(_) => thread::sleep(Duration::MAX),
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}

fn run(config: Config) -> Result<Box<dyn SerialPort>, Box<dyn Error>> {
    let data = fs::read(&config.file_path)?;
    let encoder = Encoder::default();
    let encoded_data = encoder.encode(data.as_slice())?;

    let mut port = serialport::new(&config.serial_port_name, 115_200).open()?;
    port.write_all(encoded_data.as_slice())?;
    Ok(port)
}

struct Config {
    file_path: String,
    serial_port_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let serial_port_name = args[2].clone();

        Ok(Config {
            file_path,
            serial_port_name,
        })
    }
}
