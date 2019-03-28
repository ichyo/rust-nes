use nes::apu::Apu;
use nes::bus::Bus;
use nes::cartridge::Cartridge;
use nes::cpu::Cpu;
use nes::dma::Dma;
use nes::memory::Memory;
use nes::ppu::Ppu;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), String> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
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
    let mut dma = Dma::new();

    cpu.reset(&mut Bus::new(
        &cartridge, &mut wram, &mut ppu, &mut apu, &mut dma,
    ));

    let height = 240;
    let width = 256;
    let scale = 3;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Render test",
            (width * scale) as u32,
            (height * scale) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas
        .set_logical_size(width as u32, height as u32)
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'main: loop {
        loop {
            let cycle = cpu.exec(&mut Bus::new(
                &cartridge, &mut wram, &mut ppu, &mut apu, &mut dma,
            )) as usize;
            let steal = dma.transfer(&wram, &mut ppu) as usize;

            let mut vblank_nmi = false;
            let mut new_frame = false;
            for _ in 0..3 * (cycle + steal) {
                let res = ppu.exec();
                vblank_nmi |= res.vblank_nmi;
                new_frame |= res.new_frame;
            }
            if new_frame {
                break;
            }
            if vblank_nmi {
                cpu.nmi(&mut Bus::new(
                    &cartridge, &mut wram, &mut ppu, &mut apu, &mut dma,
                ));
            }
        }
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main;
            }
        }
        let rgbs = ppu.get_buffer();
        texture.update(None, rgbs, width * 3).unwrap();
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}
