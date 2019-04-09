use lazy_static::lazy_static;

#[derive(Default, Clone)]
pub struct Palettes {
    memory: [u8; 0x20],
}

impl Palettes {
    pub fn new() -> Palettes {
        Palettes::default()
    }

    pub fn load(&self, addr: u16) -> u8 {
        assert!(addr < 0x20);
        self.memory[addr as usize]
    }

    pub fn store(&mut self, addr: u16, value: u8) {
        assert!(addr < 0x20);
        self.memory[addr as usize] = value;
        if (addr % 4) == 0 {
            // Addresses $3F10/$3F14/$3F18/$3F1C
            // are mirrors of $3F00/$3F04/$3F08/$3F0C.
            self.memory[(addr ^ 0x10) as usize] = value;
        }
    }

    pub fn get_universal_background_color(&self) -> Rgb {
        COLORS[self.memory[0] as usize]
    }

    pub fn get_background_color(&self, palette_index: u8, color_index: u8) -> Rgb {
        if color_index == 0 {
            return self.get_universal_background_color();
        }
        let addr = palette_index * 4 + color_index;
        COLORS[self.memory[addr as usize] as usize]
    }

    pub fn get_sprite_color(&self, palette_index: u8, color_index: u8) -> Rgb {
        if color_index == 0 {
            return self.get_universal_background_color();
        }
        let addr = 0x10 + palette_index * 4 + color_index;
        COLORS[self.memory[addr as usize] as usize]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
