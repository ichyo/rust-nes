#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Register {
    pub A: u8,   // accumulator
    pub X: u8,   // index register
    pub Y: u8,   // index register
    pub S: u8,   // stack pointer
    pub P: u8,   // status register
    pub PC: u16, // program counter
}

impl Register {
    pub fn new() -> Self {
        Register {
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            S: 0xff,
            P: 0x24,
            PC: 0x00,
        }
    }
    pub fn carry_flag(&self) -> bool {
        self.P & 0x01 != 0
    }
    pub fn zero_flag(&self) -> bool {
        self.P & 0x02 != 0
    }
    pub fn interrupt_disable_flag(&self) -> bool {
        self.P & 0x04 != 0
    }
    pub fn decimal_mode(&self) -> bool {
        self.P & 0x08 != 0
    }
    pub fn break_command(&self) -> bool {
        self.P & 0x10 != 0
    }
    pub fn overflow_flag(&self) -> bool {
        self.P & 0x40 != 0
    }
    pub fn negative_flag(&self) -> bool {
        self.P & 0x80 != 0
    }
    pub fn set_carry_flag(&mut self, val: bool) {
        self.P = (self.P & 0xfe) | (val as u8);
    }
    pub fn set_zero_flag(&mut self, val: bool) {
        self.P = (self.P & 0xfd) | ((val as u8) << 1);
    }
    pub fn set_interrupt_disable_flag(&mut self, val: bool) {
        self.P = (self.P & 0xfb) | ((val as u8) << 2);
    }
    pub fn set_decimal_mode(&mut self, val: bool) {
        self.P = (self.P & 0xf7) | ((val as u8) << 3);
    }
    pub fn set_break_command(&mut self, val: bool) {
        self.P = (self.P & 0xef) | ((val as u8) << 4);
    }
    pub fn set_overflow_flag(&mut self, val: bool) {
        self.P = (self.P & 0xbf) | ((val as u8) << 6);
    }
    pub fn set_negative_flag(&mut self, val: bool) {
        self.P = (self.P & 0x7f) | ((val as u8) << 7);
    }
}
