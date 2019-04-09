#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
/// key for joypad
pub enum Key {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

impl Key {
    fn mask(self) -> u8 {
        let shift = match self {
            Key::A => 0,
            Key::B => 1,
            Key::Select => 2,
            Key::Start => 3,
            Key::Up => 4,
            Key::Down => 5,
            Key::Left => 6,
            Key::Right => 7,
        };
        1 << shift
    }
}

/// State of keys (pressed or released)
#[derive(Clone, Copy, Default)]
pub struct KeyState(u8);

impl KeyState {
    /// Key down
    pub fn press(&mut self, key: Key) {
        self.0 |= key.mask();
    }

    /// Key up
    pub fn release(&mut self, key: Key) {
        self.0 &= !key.mask();
    }

    fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Default)]
/// NES controller
pub struct JoyPad {
    key_state: KeyState,
    buffer: u8,
    strobe_bit: bool,
}

impl JoyPad {
    /// Create new instance
    pub fn new() -> JoyPad {
        JoyPad::default()
    }

    /// Key down
    pub fn press(&mut self, key: Key) {
        self.key_state.press(key);
    }

    /// Key up
    pub fn release(&mut self, key: Key) {
        self.key_state.release(key);
    }

    /// Set key state
    pub fn set_key_state(&mut self, key_state: KeyState) {
        self.key_state = key_state;
    }

    /// Load via memory map
    pub fn load(&mut self) -> u8 {
        let result = self.buffer & 0x1;
        self.buffer >>= 1;
        self.update_buffer();
        result
    }

    /// Store via memory map
    pub fn store(&mut self, value: u8) {
        self.strobe_bit = value != 0;
        self.update_buffer();
    }

    fn update_buffer(&mut self) {
        if self.strobe_bit {
            self.buffer = self.key_state.as_u8();
        }
    }
}
