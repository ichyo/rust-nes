use nes::apu::Apu;
use nes::bus::Bus;
use nes::cartridge::Cartridge;
use nes::cpu::Cpu;
use nes::dma::Dma;
use nes::joypad::JoyPad;
use nes::memory::Memory;
use nes::ppu::Ppu;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() -> Result<(), String> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
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
    let mut joypad = JoyPad::new();
    let mut dma = Dma::new();

    cpu.reset(&mut Bus::new(
        &cartridge,
        &mut wram,
        &mut ppu,
        &mut apu,
        &mut joypad,
        &mut dma,
    ));

    let mut frame_count = 0;
    let start = Instant::now();

    loop {
        let cycle = cpu.exec(&mut Bus::new(
            &cartridge,
            &mut wram,
            &mut ppu,
            &mut apu,
            &mut joypad,
            &mut dma,
        )) as usize;
        let steal = dma.transfer(&wram, &mut ppu) as usize;

        let mut vblank_nmi = false;
        let mut new_frame = false;
        for _ in 0..3 * (cycle + steal) {
            let res = ppu.exec();
            vblank_nmi |= res.vblank_nmi;
            new_frame |= res.new_frame;
        }
        for _ in 0..(cycle + steal) {
            apu.tick();
        }
        if new_frame {
            frame_count += 1;
            if frame_count % 100 == 0 {
                println!(
                    "{} fps",
                    frame_count as f64 / (start.elapsed().as_nanos() as f64 / 1_000_000_000.0)
                );
            }
        }
        if vblank_nmi {
            cpu.nmi(&mut Bus::new(
                &cartridge,
                &mut wram,
                &mut ppu,
                &mut apu,
                &mut joypad,
                &mut dma,
            ));
        }
    }

    Ok(())
}
