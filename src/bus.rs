use crate::apu::Apu;
use crate::cartridge::Cartridge;
use crate::dma::Dma;
use crate::memory::Memory;
use crate::ppu::Ppu;

/// Memory map for cpu
pub struct Bus<'a> {
    cartridge: &'a Cartridge,
    wram: &'a mut Memory,
    ppu: &'a mut Ppu,
    apu: &'a mut Apu,
    dma: &'a mut Dma,
}

impl<'a> Bus<'a> {
    /// Create bus from nesessary components
    pub fn new(
        cartridge: &'a Cartridge,
        wram: &'a mut Memory,
        ppu: &'a mut Ppu,
        apu: &'a mut Apu,
        dma: &'a mut Dma,
    ) -> Bus<'a> {
        Bus {
            cartridge,
            wram,
            ppu,
            apu,
            dma,
        }
    }

    /// Load 1 byte from address
    pub fn load(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000...0x1fff => self.wram.load(addr & 0x07ff),
            0x2000...0x3fff => self.ppu.load((addr - 0x2000) & 0x7),
            0x4000...0x4015 | 0x4018...0x401f => self.apu.load(addr - 0x4000),
            0x4016 | 0x4017 => 0, // TODO: implement key pad
            0x4020...0x7fff => panic!("not implemented to load {:#x}", addr),
            0x8000...0xffff => self.cartridge.prg_rom[(addr - 0x8000) as usize],
        }
    }

    /// Store 1 byte value into address
    pub fn store(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x1fff => self.wram.store(addr & 0x7ff, val), // TODO: correct for mirror mode?
            0x2000...0x3fff => self.ppu.store((addr - 0x2000) & 0x7, val),
            0x4014 => self.dma.write(val),
            0x4000...0x4013 | 0x4015 | 0x4018...0x401f => self.apu.store(addr - 0x4000, val),
            0x4016 | 0x4017 => {} // TODO: implement key pad
            0x4020...0xffff => panic!("not implemented to set {:#x} at address {:#x}", val, addr),
        };
    }

    /// Load 2 bytes from address
    pub fn load_w(&mut self, addr: u16) -> u16 {
        // From nestest, it seems page boundary wraps next address.
        let next_addr = if addr == 0xff || addr == 0x02ff {
            addr ^ 0xff
        } else {
            addr + 1
        };
        u16::from(self.load(addr)) | (u16::from(self.load(next_addr)) << 8)
    }

    /// Store 2 bytes value into address with little endian.
    pub fn store_w(&mut self, addr: u16, val: u16) {
        // From nestest, it seems page boundary wraps next address.
        let next_addr = if addr == 0xff || addr == 0x02ff {
            addr ^ 0xff
        } else {
            addr + 1
        };
        self.store(addr, (val & 0xff) as u8);
        self.store(next_addr, (val >> 8) as u8);
    }
}
