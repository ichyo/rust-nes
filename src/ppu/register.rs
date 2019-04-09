use super::pattern::PatternTableSide;

#[derive(Debug, Clone, Copy)]
pub struct PPUCtrl {
    value: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct PPUMask {
    value: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct PPUStatus {
    value: u8,
}

impl PPUCtrl {
    pub fn new() -> PPUCtrl {
        PPUCtrl { value: 0 }
    }

    pub fn addr_incr(&self) -> u8 {
        if (self.value & 0x4) == 0 {
            1
        } else {
            32
        }
    }

    pub fn vblank_nmi(&self) -> bool {
        (self.value & 0x80) != 0
    }

    pub fn scroll_x_bit8(&self) -> u8 {
        (self.value & 0x01)
    }

    pub fn scroll_y_bit8(&self) -> u8 {
        ((self.value >> 1) & 0x01)
    }

    pub fn background_table(&self) -> PatternTableSide {
        if (self.value & 0x10) != 0 {
            PatternTableSide::Right
        } else {
            PatternTableSide::Left
        }
    }

    pub fn sprite_table(&self) -> PatternTableSide {
        if (self.value & 0x08) != 0 {
            PatternTableSide::Right
        } else {
            PatternTableSide::Left
        }
    }

    pub fn set_u8(&mut self, value: u8) {
        self.value = value;
    }
}

impl PPUMask {
    pub fn new() -> PPUMask {
        PPUMask { value: 0 }
    }

    pub fn gray_scale(&self) -> bool {
        (self.value & 0x1) != 0
    }

    pub fn show_background(&self) -> bool {
        (self.value & 0x8) != 0
    }

    pub fn show_sprite(&self) -> bool {
        (self.value & 0x10) != 0
    }

    pub fn set_u8(&mut self, value: u8) {
        self.value = value;
    }
}

impl PPUStatus {
    pub fn new() -> PPUStatus {
        PPUStatus { value: 0 }
    }

    pub fn set_sprite_0_hit(&mut self, value: bool) {
        if value {
            self.value |= 1 << 6;
        } else {
            self.value &= !(1 << 6);
        }
    }

    pub fn set_vblank(&mut self, value: bool) {
        if value {
            self.value |= 1 << 7;
        } else {
            self.value &= !(1 << 7);
        }
    }

    pub fn to_u8(&self) -> u8 {
        self.value
    }
}
