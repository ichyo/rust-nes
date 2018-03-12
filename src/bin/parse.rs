extern crate nes;

use std::env;
use std::io::Read;
use std::error::Error;
use nes::cartridge::Cartridge;

fn run() -> Result<(), Box<Error>> {
    let path = env::args().nth(1).ok_or("no arguments")?;
    let mut f = std::fs::File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf);
    let cartridge = Cartridge::parse(&buf)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e)
    }
}