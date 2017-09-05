use memory::Memory;

struct Register {
    A: u8, // accumulator
    X: u8, // index register
    Y: u8, // index register
    S: u8, // stack pointer
    P: u8, // status register
    PC: u16 // program counter
}

impl Register {
    fn carry(&self) -> u8 {
        self.P & 0x01
    }
    fn zero(&self) -> u8 {
        self.P & 0x02 >> 1
    }
    fn irq(&self) -> u8 {
        self.P & 0x04 >> 2
    }
    fn decimal_mode(&self) -> u8 {
        self.P & 0x08 >> 3
    }
    fn break_mode(&self) -> u8 {
        self.P & 0x10 >> 4
    }
    fn over_flow(&self) -> u8 {
        self.P & 0x40 >> 6
    }
    fn negative(&self) -> u8 {
        self.P & 0x80 >> 7
    }
    fn set_flag(
        &mut self,
        carry: Option<bool>,
        zero: Option<bool>,
        irq: Option<bool>,
        deciaml_mode: Option<bool>,
        break_mode: Option<bool>,
        over_flow: Option<bool>,
        negative: Option<bool>,
    ) {
        let carry = carry.unwrap_or_else(|| self.carry() == 1) as u8;
        let zero = zero.unwrap_or_else(|| self.zero() == 1) as u8;
        let irq = irq.unwrap_or_else(|| self.irq() == 1) as u8;
        let deciaml_mode = deciaml_mode.unwrap_or_else(|| self.decimal_mode() == 1) as u8;
        let break_mode = break_mode.unwrap_or_else(|| self.decimal_mode() == 1) as u8;
        let over_flow = over_flow.unwrap_or_else(|| self.over_flow() == 1) as u8;
        let negative = negative.unwrap_or_else(|| self.negative() == 1) as u8;
        self.P = carry | (zero << 1) | (irq << 2) | (deciaml_mode << 3) | (break_mode << 4) | 0x20 | (over_flow << 6) | (negative << 7);
    }
}

enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX, // Indexed Indirect
    IndirectY, // Indirect Indexed
}

enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

enum Addr {
    None,
    Immediate(u8),
    Accumulator,
    Memory(u16),
}

pub struct Cpu<M: Memory> {
    mem: M,
    reg: Register
}

impl<M: Memory> Cpu<M> {
    fn read(&self, addr: &Addr) -> u8 {
        match *addr {
            Addr::None => unreachable!(),
            Addr::Immediate(val) => val,
            Addr::Accumulator => self.reg.A,
            Addr::Memory(addr) => self.mem.read(addr),
        }
    }
    fn write(&mut self, addr: &Addr, val: u8) {
        match *addr {
            Addr::None => unreachable!(),
            Addr::Immediate(val) => unreachable!(),
            Addr::Accumulator => self.reg.A = val,
            Addr::Memory(addr) => self.mem.write(addr, val),
        }
    }
    fn exec(&mut self, op: Opcode, addr: Addr) {
        match op {
            LDA => {
                self.reg.A = self.read(&addr);
            },
            LDX => {
                self.reg.X = self.read(&addr);
            },
            LDY => {
                self.reg.Y = self.read(&addr);
            },
            STX => {
                let X = self.reg.X;
                self.write(&addr, X);
            },
            STY => {
                let Y = self.reg.Y;
                self.write(&addr, Y);
            },
            TAX => {
                self.reg.X = self.reg.A;
            },
            TAY => {
                self.reg.Y = self.reg.A;
            },
            TSX => {
                self.reg.X = self.reg.S;
            }
            TXA => {
                self.reg.A = self.reg.X;
            },
            TXS => {
                self.reg.S = self.reg.X;
            },
            TYA => {
                self.reg.A = self.reg.Y;
            },
            // TODO: set flags
            ADC => {
                self.reg.A = self.reg.A + self.read(&addr) + self.reg.carry();
            },
            AND => {
                self.reg.A = self.reg.A & self.read(&addr);
            }
            ASL => {
                let val = self.read(&addr);
                self.write(&addr, val << 1);
            }
            BIT => {

            }
        }
    }
}

