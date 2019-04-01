use nes::apu::Apu;
use nes::bus::Bus;
use nes::cartridge::Cartridge;
use nes::cpu::Cpu;
use nes::dma::Dma;
use nes::joypad::{JoyPad, Key};
use nes::memory::Memory;
use nes::ppu::Ppu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::env;
use std::fs::File;
use std::io::prelude::*;

const SAMPLE_RATE: f64 = 44_100.0;
const CHANNELS: i32 = 1;
const FRAMES: u32 = (44100.0 / 60.0) as u32;
const INTERLEAVED: bool = true;

fn joypad_key(keycode: Keycode) -> Option<Key> {
    match keycode {
        Keycode::Return => Some(Key::Start),
        Keycode::RShift => Some(Key::Select),
        Keycode::Z => Some(Key::A),
        Keycode::X => Some(Key::B),
        Keycode::Up => Some(Key::Up),
        Keycode::Down => Some(Key::Down),
        Keycode::Left => Some(Key::Left),
        Keycode::Right => Some(Key::Right),
        _ => None,
    }
}

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

    let pa = portaudio::PortAudio::new().unwrap();
    let output = pa.default_output_device().unwrap();
    let output_info = pa.device_info(output).unwrap();
    let latency = output_info.default_low_output_latency;
    let params = portaudio::StreamParameters::<f32>::new(output, CHANNELS, INTERLEAVED, latency);
    let settings = portaudio::OutputStreamSettings::new(params, SAMPLE_RATE, FRAMES);

    let mut time = 0.0;

    let mut stream = pa.open_blocking_stream(settings).unwrap();
    stream.start().unwrap();

    'main: loop {
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
            if new_frame {
                break;
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
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = joypad_key(keycode) {
                        joypad.press(key);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = joypad_key(keycode) {
                        joypad.release(key);
                    }
                }
                _ => {}
            }
            if let Event::Quit { .. } = event {
                break 'main;
            }
        }
        //let rgbs = ppu.get_buffer();
        let hz1 = apu.frequency1();
        let hz2 = apu.frequency2();
        let hz3 = apu.frequency3();
        stream.write(FRAMES, |buffer| {
            for i in 0..buffer.len() {
                let unit1 = 1.0 / hz1;
                let unit2 = 1.0 / hz2;
                let unit3 = 1.0 / hz3;
                let th1 = (time / unit1).fract();
                let th2 = (time / unit2).fract();
                let th3 = (time / unit3).fract();
                buffer[i as usize] = 0.0;
                buffer[i as usize] += 0.2 * (if th1 < 0.5 { 1.0 } else { -1.0 }) as f32;
                buffer[i as usize] += 0.2 * (if th2 < 0.5 { 1.0 } else { -1.0 }) as f32;
                buffer[i as usize] += 0.5
                    * (if th3 < 0.5 {
                        -1.0 + 4.0 * th3
                    } else {
                        -1.0 + 4.0 * (1.0 - th3)
                    }) as f32;
                time += 1.0 / SAMPLE_RATE;
            }
        });
        let rgbs = ppu.get_buffer();
        texture.update(None, rgbs, width * 3).unwrap();
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}
