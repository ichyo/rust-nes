mod background;
mod palette;
mod pattern;
mod register;
mod sprite;

use crate::cartridge::Cartridge;
use background::NameTables;
use log::{trace, warn};
use palette::{Palettes, Rgb};
use pattern::PatternTables;
use register::{PPUCtrl, PPUMask, PPUStatus};
use sprite::Sprite;

/// Picture Processing Unit. handle graphics.
pub struct Ppu {
    reg_ctrl: PPUCtrl,
    reg_mask: PPUMask,
    reg_status: PPUStatus,
    pattern_tables: PatternTables,
    name_table: NameTables,
    palette_table: Palettes,
    vram_addr: u16,
    oam_data: [u8; 0x100],
    oam_addr: u8,
    scroll: (u8, u8),
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
            self.render_line(self.scanline as u8);
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

    fn render_line(&mut self, y: u8) {
        // TODO: Use sprite priorities
        let sprites: Vec<Sprite> = self
            .get_sprites()
            .into_iter()
            .filter(|s| s.is_render_line(y))
            .collect::<Vec<_>>();
        for x in 0..WINDOW_WIDTH {
            let sprite0 = Sprite::new(&self.oam_data[0..4]);
            let sprite0_color = sprite0.get_color(
                x as u8,
                y as u8,
                self.pattern_tables.get_table(self.reg_ctrl.sprite_table()),
                &self.palette_table,
            );
            let bg_color = self.get_background_color(x as u8, y);

            // TODO: sprite 0 condition is more complex.
            // See https://wiki.nesdev.com/w/index.php/PPU_OAM#Sprite_zero_hits<Paste>
            if let (Some(_), Some(_)) = (sprite0_color, bg_color) {
                self.reg_status.set_sprite_0_hit(true);
            }

            let rgb = self
                .get_sprite_color(&sprites, x as u8, y)
                .or(bg_color)
                .unwrap_or_else(|| self.palette_table.get_universal_background_color());
            let index = 3 * (x + y as usize * WINDOW_WIDTH);
            self.render_buffer[index] = rgb.r;
            self.render_buffer[index + 1] = rgb.g;
            self.render_buffer[index + 2] = rgb.b;
        }
    }

    fn get_sprite_color(&self, sprites: &[Sprite], x: u8, y: u8) -> Option<Rgb> {
        // TODO: Read mask for
        // "Show sprites in leftmost 8 pixels of screen, 0: Hide"
        if !self.reg_mask.show_sprite() {
            return None;
        }
        sprites
            .iter()
            .filter_map(|s| {
                s.get_color(
                    x,
                    y,
                    &self.pattern_tables.get_table(self.reg_ctrl.sprite_table()),
                    &self.palette_table,
                )
            })
            .next()
    }

    fn get_background_color(&self, x: u8, y: u8) -> Option<Rgb> {
        // TODO: Read mask for
        // "Show sprites in leftmost 8 pixels of screen, 0: Hide"
        if !self.reg_mask.show_background() {
            return None;
        }
        let (scroll_x, scroll_y) = self.get_scroll();
        let (x, y) = (scroll_x + u16::from(x), scroll_y + u16::from(y));
        let pattern_index = self.name_table.get_pattern_index(x, y);
        let palette_index = self.name_table.get_palette_index(x, y);
        let sprite_value = self
            .pattern_tables
            .get_table(self.reg_ctrl.background_table())
            .get_value(pattern_index, (x % 8) as u8, (y % 8) as u8);
        let rgb = self
            .palette_table
            .get_background_color(palette_index, sprite_value);
        Some(rgb)
    }

    fn get_sprites(&self) -> Vec<Sprite> {
        (0..64)
            .map(|idx| Sprite::new(&self.oam_data[idx * 4..(idx + 1) * 4]))
            .collect::<Vec<Sprite>>()
    }

    fn get_scroll(&self) -> (u16, u16) {
        let (low_x, low_y) = self.scroll;
        let high_x = self.reg_ctrl.scroll_x_bit8();
        let high_y = self.reg_ctrl.scroll_y_bit8();
        (
            (u16::from(high_x) << 8) | u16::from(low_x),
            (u16::from(high_y) << 8) | u16::from(low_y),
        )
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
            oam_data: [0; 0x100],
            oam_addr: 0,
            scroll: (0, 0),
            pattern_tables: PatternTables::new(chr_rom),
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
            0x02 => {
                let status = self.reg_status.to_u8();
                self.reg_status.set_vblank(false);
                status
            }
            0x03 => {
                warn!("It doesn't support load oam addr (0x03)");
                0
            }
            0x04 => {
                // TODO: The actual behavior is much more complex.
                // See https://wiki.nesdev.com/w/index.php/PPU_registers#OAMDATA
                self.oam_data[self.oam_addr as usize]
            }
            0x05 => {
                warn!("It doesn't support load ppu scroll (0x05)");
                0
            }
            0x06 => {
                warn!("It doesn't support load ppu addr (0x06)");
                0
            }
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
            0x03 => {
                self.oam_addr = val;
            }
            0x04 => {
                // TODO: The actual behavior is much more complex.
                // See https://wiki.nesdev.com/w/index.php/PPU_registers#OAMDATA
                self.write_oam(val);
            }
            0x05 => {
                // TODO: What if it's during rendering?
                // https://wiki.nesdev.com/w/index.php/PPU_registers#Scroll_.28.242005.29_.3E.3E_write_x2
                if let (0x05, x) = self.last_store {
                    self.scroll = (x, val);
                }
            }
            0x06 => {
                if let (0x06, high) = self.last_store {
                    self.vram_addr = (u16::from(high) << 8) | u16::from(val);
                }
            }
            0x07 => {
                self.store_vram(self.vram_addr, val);
                self.vram_addr += u16::from(self.reg_ctrl.addr_incr());
            }
            0x08...0xffff => panic!("Unknown address {}", addr),
        };
        self.last_store = (addr, val);
    }

    /// Write value to OAM. Increment OAM address.
    pub fn write_oam(&mut self, val: u8) {
        self.oam_data[self.oam_addr as usize] = val;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    fn load_vram(&self, addr: u16) -> u8 {
        trace!("Load(vram) addr={:#x}", addr);
        match addr {
            0x0000...0x1fff => self.pattern_tables.load(addr),
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
