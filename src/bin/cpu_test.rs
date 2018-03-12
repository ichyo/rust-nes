extern crate nes;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use nes::cartridge::Cartridge;
use nes::memory::Memory;
use nes::cpu::{Bus, Cpu};

fn main() {
    let path = env::args().nth(1).expect("please specify the path to nes");
    let mut f = File::open(path).expect("failed to open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("failed to read a file");
    let cartridge = Cartridge::parse(&buffer).expect("invalid nes format");
    let mut wram = Memory::new();
    let bus = Bus::new(&cartridge, &mut wram);
    let mut cpu = Cpu::new(bus);
    cpu.reset();
    for _ in 0..100 {
        cpu.exec();
    }
}
