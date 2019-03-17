use log::{error, trace, warn};

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
        assert_eq!(chr_rom.len(), 0x2000);
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

    pub fn load(&mut self, addr: u16) -> u8 {
        eprintln!("[Ppu] load addr={:#x}", addr);
        match addr {
            0x00 => {
                eprintln!("Warning: doesn't support load control register (0x00)");
                self.reg_ctrl
            }
            0x01 => {
                eprintln!("Warning: doesn't support load mask register (0x01)");
                self.reg_mask
            }
            0x02...0x06 => 0,
            0x07 => {
                let result = self.load_vram(self.vram_addr);
                self.vram_addr += self.get_addr_incr();
                result
            }
            0x08...0xffff => panic!("Unknown address {}", addr),
        }
    }

    pub fn store(&mut self, addr: u16, val: u8) {
        trace!("Store addr={:#x} val={:#x}", addr, val);
        match addr {
            0x00 => {
                self.reg_ctrl = val;
            }
            0x01 => {
                self.reg_mask = val;
            }
            0x02 => {
                warn!("It doesn't support write to status register");
            }
            0x03...0x05 => {}
            0x06 if self.last_store.0 == 0x06 => {
                self.vram_addr = (u16::from(self.last_store.1) << 8) | u16::from(val);
            }
            0x06 => {} // TODO: unexpected?
            0x07 => {
                self.store_vram(self.vram_addr, val);
                self.vram_addr += self.get_addr_incr();
            }
            0x08...0xffff => panic!("Unknown address {}", addr),
        };
        self.last_store = (addr, val);
    }

    fn get_addr_incr(&self) -> u16 {
        match self.reg_ctrl & 0x4 {
            0 => 1,
            _ => 32,
        }
    }

    fn load_vram(&self, addr: u16) -> u8 {
        trace!("Load(vram) addr={:#x}", addr);
        match addr {
            0x0000...0x1fff => self.pattern_table[addr as usize],
            0x2000...0x2fff => self.name_table[(addr - 0x2000) as usize],
            0x3000...0x3eff => self.name_table[(addr - 0x3000) as usize],
            0x3f00...0x3fff => self.palette_table[(addr & 0x1f) as usize],
            0x4000...0xffff => unreachable!(),
        }
    }

    fn store_vram(&mut self, addr: u16, val: u8) {
        trace!("Store(vram) addr = {:#x} val = {:#x}", addr, val);
        match addr {
            0x0000...0x1fff => error!("it doesn't support to write to pattern table"),
            0x2000...0x2fff => self.name_table[(addr - 0x2000) as usize] = val,
            0x3000...0x3eff => self.name_table[(addr - 0x3000) as usize] = val,
            0x3f00...0x3fff => self.palette_table[(addr & 0x1f) as usize] = val,
            0x4000...0xffff => unreachable!(),
        }
    }
}
