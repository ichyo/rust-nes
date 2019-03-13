use log::{error, log, trace, warn};

pub struct Ppu {
    reg_ctrl: u8,
    reg_mask: u8,
    last_store: (u16, u8),
    vram_addr: u16,
    pattern_table: [u8; 0x2000],
    name_table: [u8; 0x1000],
    palette_table: [u8; 0x20],
}

impl Ppu {
    pub fn new(chr_rom: &[u8]) -> Ppu {
        assert!(chr_rom.len() == 0x2000);
        let mut pattern_table = [0; 0x2000];
        pattern_table.clone_from_slice(chr_rom);
        Ppu {
            reg_ctrl: 0,
            reg_mask: 0,
            last_store: (0, 0),
            vram_addr: 0,
            pattern_table,
            name_table: [0; 0x1000],
            palette_table: [0; 0x20],
        }
    }

    pub fn load(&self, addr: u16) -> u8 {
        eprintln!("[Ppu] load addr={:#x}", addr);
        if addr == 0x00 {
            eprintln!("Warning: doesn't support load control register (0x00)");
            self.reg_ctrl
        } else if addr == 0x01 {
            eprintln!("Warning: doesn't support load mask register (0x01)");
            self.reg_mask
        } else if addr == 0x02 {
            0
        } else if addr == 0x03 {
            0
        } else if addr == 0x04 {
            0
        } else if addr == 0x05 {
            0
        } else if addr == 0x06 {
            0
        } else if addr == 0x07 {
            0
        } else {
            panic!("Unknown address {}", addr);
        }
    }

    pub fn store(&mut self, addr: u16, val: u8) {
        trace!("Store addr={:#x} val={:#x}", addr, val);
        if addr == 0x00 {
            self.reg_ctrl = val;
        } else if addr == 0x01 {
            self.reg_mask = val;
        } else if addr == 0x02 {
            warn!("It doesn't support write to status register");
        } else if addr == 0x03 {
        } else if addr == 0x04 {
        } else if addr == 0x05 {
        } else if addr == 0x06 {
            if self.last_store.0 == 0x06 {
                self.vram_addr = ((self.last_store.1 as u16) << 8) | val as u16;
            }
        } else if addr == 0x07 {
            let vaddr = self.vram_addr;
            self.store_vram(vaddr, val);
            self.vram_addr += self.get_addr_incr();
        }
        self.last_store = (addr, val);
    }

    fn get_addr_incr(&self) -> u16 {
        if self.reg_ctrl & 0x4 != 0 {
            32
        } else {
            1
        }
    }

    fn load_vram(&self, addr: u16) -> u8 {
        trace!("Load(vram) addr={:#x}", addr);
        if addr < 0x2000 {
            self.pattern_table[addr as usize]
        } else if addr < 0x3000 {
            self.name_table[(addr - 0x2000) as usize]
        } else if addr < 0x4000 {
            self.name_table[(addr - 0x3000) as usize]
        } else {
            self.palette_table[(addr & 0x1f) as usize]
        }
    }

    fn store_vram(&mut self, addr: u16, val: u8) {
        trace!("Store(vram) addr = {:#x} val = {:#x}", addr, val);
        if addr < 0x2000 {
            error!("it doesn't support to write to pattern table");
        } else if addr < 0x3000 {
            self.name_table[(addr - 0x2000) as usize] = val;
        } else if addr < 0x4000 {
            self.name_table[(addr - 0x3000) as usize] = val;
        } else {
            self.palette_table[(addr & 0x1f) as usize] = val;
        }
    }
}
