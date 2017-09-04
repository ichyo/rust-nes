use memory::Memory;

struct Register {
    A: u8, // accumulator
    X: u8, // index register
    Y: u8, // index register
    S: u8, // stack pointer
    P: u8, // status register
    PC: u16 // program counter
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
    Memory(u16),
}

pub struct Cpu<M: Memory> {
    mem: M,
    reg: Register
}

impl<M: Memory> Cpu<M> {
    fn read(&self, addr: Addr) -> u8 {
        match addr {
            Addr::None => unreachable!(),
            Addr::Immediate(val) => val,
            Addr::Memory(addr) => self.mem.read(addr),
        }
    }
    fn write(&mut self, addr: Addr, val: u8) {
        match addr {
            Addr::None => unreachable!(),
            Addr::Immediate(val) => unreachable!(),
            Addr::Memory(addr) => self.mem.write(addr, val),
        }
    }
    fn exec(&mut self, op: Opcode, addr: Addr) {
        match op {
            LDA => {
                self.reg.A = self.read(addr);
            },
            LDX => {
                self.reg.X = self.read(addr);
            },
            LDY => {
                self.reg.Y = self.read(addr);
            },
            STX => {
                let X = self.reg.X;
                self.write(addr, X);
            },
            STY => {
                let Y = self.reg.Y;
                self.write(addr, Y);
            },
            TAX => {
                self.reg.X = self.reg.A;
            },
            TAY => {
                self.reg.Y = self.reg.A;
            },
        }
    }
}

