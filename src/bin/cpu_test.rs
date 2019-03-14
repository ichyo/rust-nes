extern crate env_logger;
extern crate log;
extern crate nes;

use nes::apu::Apu;
use nes::cartridge::Cartridge;
use nes::cpu::{Bus, Cpu};
use nes::memory::Memory;
use nes::ppu::Ppu;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::max())
        .init();

    let path = env::args().nth(1).expect("please specify the path to nes");
    let mut f = File::open(path).expect("failed to open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("failed to read a file");
    let cartridge = Cartridge::parse_file(&buffer).expect("invalid nes format");
    let mut wram = Memory::new();
    let mut apu = Apu::new();
    let mut ppu = Ppu::new(&cartridge.chr_rom);
    let bus = Bus::new(&cartridge, &mut wram, &mut ppu, &mut apu);
    let mut cpu = Cpu::new(bus);
    cpu.reset();
    for _ in 0..1000 {
        cpu.exec();
    }
}
