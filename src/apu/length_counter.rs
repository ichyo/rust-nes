pub struct LengthCounter {
    enabled: bool,
    halt: bool,
    counter: u8,
}

static LENGTH_TABLE: [u8; 0x20] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

impl LengthCounter {
    pub fn new() -> LengthCounter {
        LengthCounter {
            enabled: false,
            halt: false,
            counter: 0,
        }
    }

    pub fn tick(&mut self) {
        if !self.halt && self.counter > 0 {
            self.counter -= 1;
        }
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }

    pub fn load_with_index(&mut self, length_index: u8) {
        self.counter = LENGTH_TABLE[length_index as usize]
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !self.enabled {
            self.counter = 0;
        }
    }

    pub fn set_halt(&mut self, halt: bool) {
        self.halt = halt;
    }
}
