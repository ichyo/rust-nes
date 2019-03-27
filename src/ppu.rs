mod background;
mod palette;
mod pattern;
mod register;

use crate::cartridge::Cartridge;
use background::NameTables;
use log::{error, trace, warn};
use palette::Palettes;
use pattern::PatternTable;
use register::{PPUCtrl, PPUMask, PPUStatus};

/// Picture Processing Unit. handle graphics.
pub struct Ppu {
    reg_ctrl: PPUCtrl,
    reg_mask: PPUMask,
    reg_status: PPUStatus,
    pattern_table: PatternTable,
    name_table: NameTables,
    palette_table: Palettes,
    vram_addr: u16,
    last_store: (u16, u8),
    scanline: u16,
    cycles_in_line: u16,
    ppudata_buffer: u8,
    render_buffer: [u8; WINDOW_HEIGHT * WINDOW_WIDTH * 3],
}

/// Store execution result in a cycle
pub struct ExecResult {
    /// true if nmi interupt should happen
    pub vblank_nmi: bool,
    /// true if new frame starts
    pub new_frame: bool,
}

impl ExecResult {
    fn new() -> ExecResult {
        ExecResult {
            vblank_nmi: false,
            new_frame: false,
        }
    }
}

const TOTAL_SCANLINE: u16 = 262;
const TOTAL_CYCLES_IN_LINE: u16 = 341;
const WINDOW_HEIGHT: usize = 240;
const WINDOW_WIDTH: usize = 256;

impl Ppu {
    /// get render buffer
    pub fn get_buffer(&self) -> &[u8] {
        &self.render_buffer
    }

    /// Execute single cycle
    pub fn exec(&mut self) -> ExecResult {
        let mut result = ExecResult::new();

        self.cycles_in_line = (self.cycles_in_line + 1) % TOTAL_CYCLES_IN_LINE;
        if self.cycles_in_line != 0 {
            return result;
        }

        if self.scanline < WINDOW_HEIGHT as u16 {
            self.render_line(self.scanline as usize);
        }
        if self.scanline == WINDOW_HEIGHT as u16 {
            self.reg_status.set_vblank(true);
            if self.reg_ctrl.vblank_nmi() {
                result.vblank_nmi = true;
            }
        }
        if self.scanline == TOTAL_SCANLINE - 1 {
            self.reg_status.set_vblank(false);
            self.reg_status.set_sprite_0_hit(false);
            result.new_frame = true;
        }

        self.scanline = (self.scanline + 1) % TOTAL_SCANLINE;
        result
    }

    fn render_line(&mut self, y: usize) {
        if !self.reg_mask.show_background() {
            return;
        }
        for x in 0..WINDOW_WIDTH {
            let pattern_index = self.name_table.get_pattern_index(x as u16, y as u16);
            let palette_index = self.name_table.get_palette_index(x as u16, y as u16);
            let sprite_value = self.pattern_table.get_value(
                self.reg_ctrl.background_table(),
                pattern_index,
                (x % 8) as u8,
                (y % 8) as u8,
            );
            let rgb = self
                .palette_table
                .get_background_color(palette_index, sprite_value);
            let index = 3 * (x + y * WINDOW_WIDTH);
            self.render_buffer[index] = rgb.r;
            self.render_buffer[index + 1] = rgb.g;
            self.render_buffer[index + 2] = rgb.b;
        }
    }

    /// Create PPU from cartridge
    pub fn from_cartridge(cartridge: &Cartridge) -> Ppu {
        Ppu::new(&cartridge.chr_rom)
    }

    fn new(chr_rom: &[u8]) -> Ppu {
        Ppu {
            reg_ctrl: PPUCtrl::new(),
            reg_mask: PPUMask::new(),
            reg_status: PPUStatus::new(),
            last_store: (0, 0),
            vram_addr: 0,
            pattern_table: PatternTable::new(chr_rom),
            name_table: NameTables::new(),
            palette_table: Palettes::new(),
            scanline: 0,
            cycles_in_line: 0,
            ppudata_buffer: 0,
            render_buffer: [0; WINDOW_HEIGHT * WINDOW_WIDTH * 3],
        }
    }

    /// load interface exposed to cpu via bus
    pub fn load(&mut self, addr: u16) -> u8 {
        trace!("[Ppu] load addr={:#x}", addr);
        match addr {
            0x00 => {
                warn!("It doesn't support load control register (0x00)");
                0
            }
            0x01 => {
                warn!("It doesn't support load mask register (0x01)");
                0
            }
            0x02 => self.reg_status.to_u8(),
            0x03...0x06 => 0,
            0x07 => {
                let buf_result = self.ppudata_buffer;
                let new_result = self.load_vram(self.vram_addr);
                let result = match self.vram_addr {
                    0x3f00...0x3fff => new_result,
                    _ => buf_result,
                };
                self.vram_addr += u16::from(self.reg_ctrl.addr_incr());
                self.ppudata_buffer = new_result;
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
                self.reg_ctrl.set_u8(val);
            }
            0x01 => {
                self.reg_mask.set_u8(val);
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
                self.vram_addr += u16::from(self.reg_ctrl.addr_incr());
            }
            0x08...0xffff => panic!("Unknown address {}", addr),
        };
        self.last_store = (addr, val);
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
            0x0000...0x1fff => warn!("it doesn't support to write to pattern table {:04X}", addr),
            0x2000...0x2fff => self.name_table.store(addr - 0x2000, val),
            0x3000...0x3eff => self.name_table.store(addr - 0x3000, val),
            0x3f00...0x3fff => self.palette_table.store(addr & 0x1f, val),
            0x4000...0xffff => unreachable!(),
        }
    }
}
