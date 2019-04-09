/// Define register model
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Register {
    pub A: u8,             // accumulator
    pub X: u8,             // index register
    pub Y: u8,             // index register
    pub S: u8,             // stack pointer
    pub P: StatusRegister, // status register
    pub PC: u16,           // program counter
}

#[derive(Debug, Clone)]
pub struct StatusRegister(u8);

impl StatusRegister {
    pub fn new() -> Self {
        // actual power-up state is 0x34 but use the same state of golden log of nestest
        StatusRegister(0x24)
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }

    pub fn set_u8(&mut self, value: u8) {
        self.0 = value | 0x20;
    }

    pub fn carry_flag(&self) -> bool {
        self.0 & 0x01 != 0
    }
    pub fn zero_flag(&self) -> bool {
        self.0 & 0x02 != 0
    }
    pub fn interrupt_disable_flag(&self) -> bool {
        self.0 & 0x04 != 0
    }
    pub fn decimal_mode(&self) -> bool {
        self.0 & 0x08 != 0
    }
    pub fn break_command(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn overflow_flag(&self) -> bool {
        self.0 & 0x40 != 0
    }
    pub fn negative_flag(&self) -> bool {
        self.0 & 0x80 != 0
    }
    pub fn set_carry_flag(&mut self, val: bool) {
        self.0 = (self.0 & 0xfe) | (val as u8);
    }
    pub fn set_zero_flag(&mut self, val: bool) {
        self.0 = (self.0 & 0xfd) | ((val as u8) << 1);
    }
    pub fn set_interrupt_disable_flag(&mut self, val: bool) {
        self.0 = (self.0 & 0xfb) | ((val as u8) << 2);
    }
    pub fn set_decimal_mode(&mut self, val: bool) {
        self.0 = (self.0 & 0xf7) | ((val as u8) << 3);
    }
    pub fn set_break_command(&mut self, val: bool) {
        self.0 = (self.0 & 0xef) | ((val as u8) << 4);
    }
    pub fn set_overflow_flag(&mut self, val: bool) {
        self.0 = (self.0 & 0xbf) | ((val as u8) << 6);
    }
    pub fn set_negative_flag(&mut self, val: bool) {
        self.0 = (self.0 & 0x7f) | ((val as u8) << 7);
    }
}

impl Default for StatusRegister {
    fn default() -> StatusRegister {
        StatusRegister::new()
    }
}

impl Default for Register {
    fn default() -> Register {
        Register::new()
    }
}

impl Register {
    pub fn new() -> Self {
        Register {
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            S: 0xfd,
            P: StatusRegister::new(),
            PC: 0xc000,
        }
    }
}
