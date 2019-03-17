use lazy_static::lazy_static;
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

#[derive(Clone, Copy)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }
}

lazy_static! {
    static ref COLORS: [Rgb; 64] = [
        Rgb::new(124, 124, 124),
        Rgb::new(0, 0, 252),
        Rgb::new(0, 0, 188),
        Rgb::new(68, 40, 188),
        Rgb::new(148, 0, 132),
        Rgb::new(168, 0, 32),
        Rgb::new(168, 16, 0),
        Rgb::new(136, 20, 0),
        Rgb::new(80, 48, 0),
        Rgb::new(0, 120, 0),
        Rgb::new(0, 104, 0),
        Rgb::new(0, 88, 0),
        Rgb::new(0, 64, 88),
        Rgb::new(0, 0, 0),
        Rgb::new(0, 0, 0),
        Rgb::new(0, 0, 0),
        Rgb::new(188, 188, 188),
        Rgb::new(0, 120, 248),
        Rgb::new(0, 88, 248),
        Rgb::new(104, 68, 252),
        Rgb::new(216, 0, 204),
        Rgb::new(228, 0, 88),
        Rgb::new(248, 56, 0),
        Rgb::new(228, 92, 16),
        Rgb::new(172, 124, 0),
        Rgb::new(0, 184, 0),
        Rgb::new(0, 168, 0),
        Rgb::new(0, 168, 68),
        Rgb::new(0, 136, 136),
        Rgb::new(0, 0, 0),
        Rgb::new(0, 0, 0),
        Rgb::new(0, 0, 0),
        Rgb::new(248, 248, 248),
        Rgb::new(60, 188, 252),
        Rgb::new(104, 136, 252),
        Rgb::new(152, 120, 248),
        Rgb::new(248, 120, 248),
        Rgb::new(248, 88, 152),
        Rgb::new(248, 120, 88),
        Rgb::new(252, 160, 68),
        Rgb::new(248, 184, 0),
        Rgb::new(184, 248, 24),
        Rgb::new(88, 216, 84),
        Rgb::new(88, 248, 152),
        Rgb::new(0, 232, 216),
        Rgb::new(120, 120, 120),
        Rgb::new(0, 0, 0),
        Rgb::new(0, 0, 0),
        Rgb::new(252, 252, 252),
        Rgb::new(164, 228, 252),
        Rgb::new(184, 184, 248),
        Rgb::new(216, 184, 248),
        Rgb::new(248, 184, 248),
        Rgb::new(248, 164, 192),
        Rgb::new(240, 208, 176),
        Rgb::new(252, 224, 168),
        Rgb::new(248, 216, 120),
        Rgb::new(216, 248, 120),
        Rgb::new(184, 248, 184),
        Rgb::new(184, 248, 216),
        Rgb::new(0, 252, 252),
        Rgb::new(248, 216, 248),
        Rgb::new(0, 0, 0),
        Rgb::new(0, 0, 0),
    ];
}

struct Sprite([[u8; 8]; 8]);

impl Sprite {
    fn new() -> Sprite {
        Sprite([[0; 8]; 8])
    }

    fn parse(chr: &[u8], index: usize) -> Sprite {
        let base = index << 4;
        let mut sprite = Sprite::new();
        for y in 0..8 {
            for x in 0..8 {
                let c1 = chr[base + y] >> (7 - x) & 1;
                let c2 = chr[base + y + 8] >> (7 - x) & 1;
                let c = (c2 << 1) | c1;
                assert!(c <= 3);
                sprite.0[y][x] = c;
            }
        }
        sprite
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }
}

const WINDOW_HEIGHT: usize = 240;
const WINDOW_WIDTH: usize = 256;

impl Ppu {
    pub fn render(&self) -> Vec<Rgb> {
        let mut res = Vec::new();
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                let by = y / 8;
                let bx = x / 8;
                let ay = y / 16;
                let ax = x / 16;

                let name_index = by * (WINDOW_WIDTH / 8) + bx;
                let pattern_index = self.name_table[name_index] as usize;

                let attr_index = 0x3c0 + ay * (WINDOW_WIDTH / 16) + ax;
                let attr_shift = 4 * (by % 2) + 2 * (bx % 2);
                let palette_index = (self.name_table[attr_index] >> attr_shift) % 0b11;

                let sprite = Sprite::parse(&self.pattern_table, pattern_index);
                let sprite_value = sprite.get(x % 8, y % 8);
                let palette_value = self.pattern_table[(palette_index * 4 + sprite_value) as usize];

                let rgb = COLORS[palette_value as usize];
                res.push(rgb);
            }
        }
        res
    }

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
