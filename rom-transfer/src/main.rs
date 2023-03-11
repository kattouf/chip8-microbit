#![feature(error_in_core)]

use std::{env, fs, process, time::Duration};
use core::error::Error;
use scp::encoder::Encoder;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&config.file_path)?;
    let encoder = Encoder::default();
    let encoded_data = encoder.encode(data.as_slice())?;

    let mut port = serialport::new(&config.serial_port_name, 115_200)
        .timeout(Duration::from_millis(10))
        .open()?;
    port.write(encoded_data.as_slice())?;

    Ok(())
}

struct Config {
    file_path: String,
    serial_port_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let serial_port_name = args[2].clone();

        Ok(Config { file_path, serial_port_name })
    }
}
