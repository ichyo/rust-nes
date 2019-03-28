use crate::memory::Memory;
use crate::ppu::Ppu;

/// DMA transfer for PPU OAM
#[derive(Default)]
pub struct Dma {
    high: Option<u8>,
}

const DMA_CYCLES: u16 = 514;

impl Dma {
    /// Create new instance
    pub fn new() -> Dma {
        Dma::default()
    }

    /// Write high byte of address for transfer
    pub fn write(&mut self, value: u8) {
        self.high = Some(value);
    }

    /// Transer 256 bytes to OAM
    pub fn transfer(&mut self, memory: &Memory, ppu: &mut Ppu) -> u16 {
        if let Some(high) = self.high {
            for low in 0..0x100 {
                let addr = (u16::from(high) << 8) | low;
                ppu.write_oam(memory.load(addr));
            }
            self.high = None;
            return DMA_CYCLES;
        }
        0
    }
}
