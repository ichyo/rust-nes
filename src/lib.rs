#![deny(clippy::all)]
#![deny(missing_docs)]

//! NES emulator written in rust

/// audio processing uni
pub mod apu;
/// memory map for cpu
pub mod bus;
/// cartridge with two memory units
pub mod cartridge;
/// processor
pub mod cpu;
/// transfer whole data from ram to PPU OAM
pub mod dma;
/// controller
pub mod joypad;
/// volatile memory
pub mod memory;
/// picture processing unit
pub mod ppu;

use crate::apu::Apu;
use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::dma::Dma;
use crate::joypad::JoyPad;
use crate::memory::Memory;
use crate::ppu::Ppu;

pub use crate::cartridge::Cartridge;
pub use crate::joypad::Key;
pub use crate::joypad::KeyState;
pub use crate::ppu::WINDOW_HEIGHT;
pub use crate::ppu::WINDOW_WIDTH;

#[derive(Clone)]
/// Main struct that contains all nes systems
pub struct Nes {
    cartridge: Cartridge,
    wram: Memory,
    apu: Apu,
    ppu: Ppu,
    cpu: Cpu,
    joypad: JoyPad,
    dma: Dma,
}

impl Nes {
    /// Construct NES object with cartridge
    pub fn with_cartridge(cartridge: Cartridge) -> Nes {
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

        Nes {
            cartridge,
            wram,
            apu,
            ppu,
            cpu,
            joypad,
            dma,
        }
    }
}

impl Nes {
    /// Executes CPU, PPU and APU until next frame comes.
    pub fn next_frame(&mut self, key_state: KeyState) {
        self.joypad.set_key_state(key_state);
        loop {
            let cycle = self.cpu.exec(&mut Bus::new(
                &self.cartridge,
                &mut self.wram,
                &mut self.ppu,
                &mut self.apu,
                &mut self.joypad,
                &mut self.dma,
            )) as usize;
            let steal = self.dma.transfer(&self.wram, &mut self.ppu) as usize;

            let mut vblank_nmi = false;
            let mut new_frame = false;
            for _ in 0..3 * (cycle + steal) {
                let res = self.ppu.exec();
                vblank_nmi |= res.vblank_nmi;
                new_frame |= res.new_frame;
            }
            for _ in 0..(cycle + steal) {
                self.apu.tick();
            }
            if new_frame {
                break;
            }
            if vblank_nmi {
                self.cpu.nmi(&mut Bus::new(
                    &self.cartridge,
                    &mut self.wram,
                    &mut self.ppu,
                    &mut self.apu,
                    &mut self.joypad,
                    &mut self.dma,
                ));
            }
        }
    }

    /// Return frame buffer in the last frame
    pub fn get_frame_buffer(&self) -> &[u8] {
        &self.ppu.get_buffer()
    }

    /// Consume audio buffer in APU
    pub fn consume_audio_buffer(&mut self) -> Vec<f32> {
        self.apu.consume_buffer().collect::<Vec<_>>()
    }
}
