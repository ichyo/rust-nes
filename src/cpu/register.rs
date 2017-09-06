#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Register {
    pub A: u8, // accumulator
    pub X: u8, // index register
    pub Y: u8, // index register
    pub S: u8, // stack pointer
    pub P: u8, // status register
    pub PC: u16 // program counter
}

impl Register {
    fn carry_flag(&self) -> bool {
        self.P & 0x01 != 0
    }
    fn zero_flag(&self) -> bool {
        self.P & 0x02 != 0
    }
    fn interrupt_disable_flag(&self) -> bool {
        self.P & 0x04 != 0
    }
    fn decimal_mode(&self) -> bool {
        self.P & 0x08 != 0
    }
    fn break_command(&self) -> bool {
        self.P & 0x10 != 0
    }
    fn overflow_flag(&self) -> bool {
        self.P & 0x40 != 0
    }
    fn negative_flag(&self) -> bool {
        self.P & 0x80 != 0
    }
    fn set_carry_flag(&mut self, val: bool) {
        self.P = (self.P & 0xfe) | (val as u8);
    }
    fn set_zero_flag(&mut self, val: bool) {
        self.P = (self.P & 0xfd) | ((val as u8) << 1);
    }
    fn set_interrupt_disable_flag(&mut self, val: bool) {
        self.P = (self.P & 0xfb) | ((val as u8) << 2);
    }
    fn set_decimal_mode(&mut self, val: bool) {
        self.P = (self.P & 0xf7) | ((val as u8) << 3);
    }
    fn set_break_command(&mut self, val: bool) {
        self.P = (self.P & 0xef) | ((val as u8) << 4);
    }
    fn set_overflow_flag(&mut self, val: bool) {
        self.P = (self.P & 0xbf) | ((val as u8) << 6);
    }
    fn set_negative_flag(&mut self, val: bool) {
        self.P = (self.P & 0x7f) | ((val as u8) << 7);
    }
}
