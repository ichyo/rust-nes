#[derive(Debug)]
pub struct PPUCtrl {
    value: u8,
}

#[derive(Debug)]
pub struct PPUMask {
    value: u8,
}

#[derive(Debug)]
pub struct PPUStatus {
    value: u8,
}

impl PPUCtrl {
    pub fn new() -> PPUCtrl {
        PPUCtrl { value: 0 }
    }

    pub fn get_addr_incr(&self) -> u8 {
        if (self.value & 0x4) == 0 {
            1
        } else {
            32
        }
    }

    pub fn get_vblank_nmi(&self) -> bool {
        (self.value & 0x80) != 0
    }

    pub fn set_u8(&mut self, value: u8) {
        self.value = value;
    }
}

impl PPUMask {
    pub fn new() -> PPUMask {
        PPUMask { value: 0 }
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
