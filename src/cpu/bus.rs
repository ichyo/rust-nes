use memory::Memory;
use cartridge::Cartridge;

pub struct Bus<'a> {
    cartridge: &'a Cartridge,
    wram: &'a mut Memory,
}

impl<'a> Bus<'a> {
    pub fn new(cartridge: &'a Cartridge, wram: &'a mut Memory) -> Bus<'a> {
        Bus { cartridge, wram }
    }

    pub fn load(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.wram.load(addr & 0x7ff) // TODO: correct for mirror mode?
        } else if addr < 0x8000 {
            eprintln!("warning: not implemented to load {:#x}", addr);
            0
        } else {
            let addr = (addr - 0x8000) as usize;
            eprintln!(
                "load prg[{:#x}] = {:#x}",
                addr, self.cartridge.prg_rom[addr]
            );
            self.cartridge.prg_rom[addr]
        }
    }

    pub fn store(&mut self, addr: u16, val: u8) {
        if addr < 0x2000 {
            self.wram.store(addr & 0x7ff, val) // TODO: correct for mirror mode?
        } else if addr < 0x8000 {
            eprintln!(
                "warning: not implemented to set {:#x} at address {:#x}",
                val, addr
            );
        } else {
            unreachable!();
        }
    }

    pub fn load_w(&self, addr: u16) -> u16 {
        self.load(addr) as u16 | ((self.load(addr + 1) as u16) << 8)
    }
    pub fn store_w(&mut self, addr: u16, val: u16) {
        self.store(addr, (val & 0xff) as u8);
        self.store(addr + 1, (val >> 8) as u8);
    }
}
