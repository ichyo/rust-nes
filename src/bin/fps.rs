use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use nes::Cartridge;
use nes::KeyState;
use nes::Nes;

fn read_cartridge(path: &str) -> Result<Cartridge, Box<Error>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let cartridge = Cartridge::parse_file(&buffer)?;
    Ok(cartridge)
}

fn main() -> Result<(), Box<Error>> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let path = env::args()
        .nth(1)
        .ok_or("please specify path to rom in argument")?;
    let cartridge = read_cartridge(&path)?;
    let mut nes = Nes::with_cartridge(cartridge);

    let mut frame_count = 0;
    let start = Instant::now();

    loop {
        nes.next_frame(KeyState::default());
        frame_count += 1;
        if frame_count % 100 == 0 {
            println!(
                "{} fps",
                f64::from(frame_count) / (start.elapsed().as_nanos() as f64 / 1_000_000_000.0)
            );
        }
    }
}
