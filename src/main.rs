use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use nes::Cartridge;
use nes::Key;
use nes::KeyState;
use nes::Nes;
use nes::WINDOW_HEIGHT;
use nes::WINDOW_WIDTH;

const SCALE: usize = 3;

const SAMPLE_RATE: f64 = 44_100.0;
const CHANNELS: i32 = 1;
const FRAMES: u32 = 1024;
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

fn read_cartridge(path: &str) -> Result<Cartridge, Box<Error>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let cartridge = Cartridge::parse_file(&buffer)?;
    Ok(cartridge)
}

fn create_canvas(sdl: &Sdl) -> Result<WindowCanvas, Box<Error>> {
    let video_subsystem = sdl.video()?;

    let window = video_subsystem
        .window(
            "Render test",
            (WINDOW_WIDTH * SCALE) as u32,
            (WINDOW_HEIGHT * SCALE) as u32,
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
        .set_logical_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .map_err(|e| e.to_string())?;

    Ok(canvas)
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

    let sdl_context = sdl2::init()?;
    let mut canvas = create_canvas(&sdl_context)?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
        )
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let pa = portaudio::PortAudio::new().unwrap();
    let output = pa.default_output_device().unwrap();
    let output_info = pa.device_info(output).unwrap();
    let latency = output_info.default_high_output_latency;
    let params = portaudio::StreamParameters::<f32>::new(output, CHANNELS, INTERLEAVED, latency);
    let settings = portaudio::OutputStreamSettings::new(params, SAMPLE_RATE, FRAMES);

    let mut stream = pa.open_blocking_stream(settings).unwrap();
    stream.start().unwrap();

    let mut key_state = KeyState::default();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = joypad_key(keycode) {
                        key_state.press(key);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = joypad_key(keycode) {
                        key_state.release(key);
                    }
                }
                _ => {}
            }
            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        nes.next_frame(key_state);

        let audio_buffer = nes.consume_audio_buffer();
        stream.write(audio_buffer.len() as u32, |output| {
            output.copy_from_slice(&audio_buffer);
        });

        let frame_buffer = nes.get_frame_buffer();
        texture
            .update(None, frame_buffer, WINDOW_WIDTH * 3)
            .unwrap();
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}
