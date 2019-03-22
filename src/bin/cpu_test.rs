use nes::apu::Apu;
use nes::bus::Bus;
use nes::cartridge::Cartridge;
use nes::cpu::Cpu;
use nes::memory::Memory;
use nes::ppu::Ppu;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, log::LevelFilter::Warn)
        .filter(Some("nes::cpu::core"), log::LevelFilter::Trace)
        .init();

    let path = env::args().nth(1).expect("please specify the path to nes");
    let mut f = File::open(path).expect("failed to open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("failed to read a file");
    let cartridge = Cartridge::parse_file(&buffer).expect("invalid nes format");
    let mut wram = Memory::new();
    let mut apu = Apu::new();
    let mut ppu = Ppu::from_cartridge(&cartridge);
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(&cartridge, &mut wram, &mut ppu, &mut apu);

    //cpu.reset(&mut bus);
    for _ in 0..100000 {
        cpu.exec(&mut bus);
    }
}
