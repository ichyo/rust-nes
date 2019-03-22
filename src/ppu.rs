mod background;
mod palette;
mod pattern;

use crate::cartridge::Cartridge;
use background::NameTables;
use log::{error, trace, warn};
use palette::Palettes;
use pattern::PatternTable;

/// Picture Processing Unit. handle graphics.
pub struct Ppu {
    reg_ctrl: u8,
    reg_mask: u8,
    last_store: (u16, u8),
    vram_addr: u16,
    pattern_table: PatternTable,
    name_table: NameTables,
    palette_table: Palettes,
}

const WINDOW_HEIGHT: usize = 240;
const WINDOW_WIDTH: usize = 256;

impl Ppu {
    /// Render method for testing. it will be removed soon.
    pub fn render(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                let pattern_index = self.name_table.get_pattern_index(x as u16, y as u16);
                let palette_index = self.name_table.get_palette_index(x as u16, y as u16);
                let sprite_value =
                    self.pattern_table
                        .get_left_value(pattern_index, (x % 8) as u8, (y % 8) as u8);
                let rgb = self
                    .palette_table
                    .get_background_color(palette_index, sprite_value);
                res.push(rgb.r);
                res.push(rgb.g);
                res.push(rgb.b);
            }
        }
        res
    }

    /// Create PPU from cartridge
    pub fn from_cartridge(cartridge: &Cartridge) -> Ppu {
        Ppu::new(&cartridge.chr_rom)
    }

    fn new(chr_rom: &[u8]) -> Ppu {
        Ppu {
            reg_ctrl: 0,
            reg_mask: 0,
            last_store: (0, 0),
            vram_addr: 0,
            pattern_table: PatternTable::new(chr_rom),
            name_table: NameTables::new(),
            palette_table: Palettes::new(),
        }
    }

    /// load interface exposed to cpu via bus
    pub fn load(&mut self, addr: u16) -> u8 {
        trace!("[Ppu] load addr={:#x}", addr);
        match addr {
            0x00 => {
                eprintln!("Warning: doesn't support load control register (0x00)");
                self.reg_ctrl
            }
            0x01 => {
                eprintln!("Warning: doesn't support load mask register (0x01)");
                self.reg_mask
            }
            0x02 => 0b1000_0000,
            0x03...0x06 => 0,
            0x07 => {
                let result = self.load_vram(self.vram_addr);
                self.vram_addr += self.get_addr_incr();
                result
            }
            0x08...0xffff => panic!("Unknown address {}", addr),
        }
    }

    /// store interface exposed to cpu via bus
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
            0x0000...0x1fff => self.pattern_table.load(addr),
            0x2000...0x2fff => self.name_table.load(addr - 0x2000),
            0x3000...0x3eff => self.name_table.load(addr - 0x3000),
            0x3f00...0x3fff => self.palette_table.load(addr & 0x1f),
            0x4000...0xffff => unreachable!(),
        }
    }

    fn store_vram(&mut self, addr: u16, val: u8) {
        trace!("Store(vram) addr = {:#x} val = {:#x}", addr, val);
        match addr {
            0x0000...0x1fff => error!("it doesn't support to write to pattern table"),
            0x2000...0x2fff => self.name_table.store(addr - 0x2000, val),
            0x3000...0x3eff => self.name_table.store(addr - 0x3000, val),
            0x3f00...0x3fff => self.palette_table.store(addr & 0x1f, val),
            0x4000...0xffff => unreachable!(),
        }
    }
}
