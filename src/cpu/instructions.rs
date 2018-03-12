use std::collections::HashMap;

pub fn lookup_instruction(code: u8) -> Instruction {
    match INSTRUCTION_SET.get(&code) {
        Some(inst) => *inst,
        None => panic!("invalid instruction code"),
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub mode: AddressingMode,
    pub steps: u8,
}

#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
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
    Relative,
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
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

lazy_static! {
    static ref INSTRUCTION_SET: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0x00, Instruction{
            opcode: Opcode::BRK,
            mode: AddressingMode::Implied,
            steps: 7
        });
        m.insert(0x01, Instruction{
            opcode: Opcode::ORA,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x05, Instruction{
            opcode: Opcode::ORA,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x06, Instruction{
            opcode: Opcode::ASL,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0x09, Instruction{
            opcode: Opcode::ORA,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0x0a, Instruction{
            opcode: Opcode::ASL,
            mode: AddressingMode::Accumulator,
            steps: 2
        });
        m.insert(0x0d, Instruction{
            opcode: Opcode::ORA,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x0e, Instruction{
            opcode: Opcode::ASL,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x10, Instruction{
            opcode: Opcode::BPL,
            mode: AddressingMode::Relative,
            steps: 2
        });
        m.insert(0x11, Instruction{
            opcode: Opcode::ORA,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0x15, Instruction{
            opcode: Opcode::ORA,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x16, Instruction{
            opcode: Opcode::ASL,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0x19, Instruction{
            opcode: Opcode::ORA,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0x1d, Instruction{
            opcode: Opcode::ORA,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0x1e, Instruction{
            opcode: Opcode::ASL,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0x20, Instruction{
            opcode: Opcode::JSR,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x21, Instruction{
            opcode: Opcode::AND,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x24, Instruction{
            opcode: Opcode::BIT,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x25, Instruction{
            opcode: Opcode::AND,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x26, Instruction{
            opcode: Opcode::ROL,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0x29, Instruction{
            opcode: Opcode::AND,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0x2a, Instruction{
            opcode: Opcode::ROL,
            mode: AddressingMode::Accumulator,
            steps: 2
        });
        m.insert(0x2c, Instruction{
            opcode: Opcode::BIT,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x2d, Instruction{
            opcode: Opcode::AND,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x2e, Instruction{
            opcode: Opcode::ROL,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x30, Instruction{
            opcode: Opcode::BMI,
            mode: AddressingMode::Relative,
            steps: 2
        });
        m.insert(0x31, Instruction{
            opcode: Opcode::AND,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0x35, Instruction{
            opcode: Opcode::AND,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x36, Instruction{
            opcode: Opcode::ROL,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0x39, Instruction{
            opcode: Opcode::AND,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0x3d, Instruction{
            opcode: Opcode::AND,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0x3e, Instruction{
            opcode: Opcode::ROL,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0x40, Instruction{
            opcode: Opcode::RTI,
            mode: AddressingMode::Implied,
            steps: 6
        });
        m.insert(0x41, Instruction{
            opcode: Opcode::EOR,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x45, Instruction{
            opcode: Opcode::EOR,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x46, Instruction{
            opcode: Opcode::LSR,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0x49, Instruction{
            opcode: Opcode::EOR,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0x4a, Instruction{
            opcode: Opcode::LSR,
            mode: AddressingMode::Accumulator,
            steps: 2
        });
        m.insert(0x4c, Instruction{
            opcode: Opcode::JMP,
            mode: AddressingMode::Absolute,
            steps: 3
        });
        m.insert(0x4d, Instruction{
            opcode: Opcode::EOR,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x4e, Instruction{
            opcode: Opcode::LSR,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x50, Instruction{
            opcode: Opcode::BVC,
            mode: AddressingMode::Relative,
            steps: 2
        });
        m.insert(0x51, Instruction{
            opcode: Opcode::EOR,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0x55, Instruction{
            opcode: Opcode::EOR,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x56, Instruction{
            opcode: Opcode::LSR,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0x59, Instruction{
            opcode: Opcode::EOR,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0x5d, Instruction{
            opcode: Opcode::EOR,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0x5e, Instruction{
            opcode: Opcode::LSR,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0x60, Instruction{
            opcode: Opcode::RTS,
            mode: AddressingMode::Implied,
            steps: 6
        });
        m.insert(0x61, Instruction{
            opcode: Opcode::ADC,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x65, Instruction{
            opcode: Opcode::ADC,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x66, Instruction{
            opcode: Opcode::ROR,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0x69, Instruction{
            opcode: Opcode::ADC,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0x6a, Instruction{
            opcode: Opcode::ROR,
            mode: AddressingMode::Accumulator,
            steps: 2
        });
        m.insert(0x6c, Instruction{
            opcode: Opcode::JMP,
            mode: AddressingMode::Indirect,
            steps: 5
        });
        m.insert(0x6d, Instruction{
            opcode: Opcode::ADC,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x6e, Instruction{
            opcode: Opcode::ROR,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x70, Instruction{
            opcode: Opcode::BCS,
            mode: AddressingMode::Relative,
            steps: 2
        });
        m.insert(0x71, Instruction{
            opcode: Opcode::ADC,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0x75, Instruction{
            opcode: Opcode::ADC,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x76, Instruction{
            opcode: Opcode::ROR,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0x79, Instruction{
            opcode: Opcode::ADC,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0x7d, Instruction{
            opcode: Opcode::ADC,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0x7e, Instruction{
            opcode: Opcode::ROR,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0x81, Instruction{
            opcode: Opcode::STA,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x84, Instruction{
            opcode: Opcode::STY,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x85, Instruction{
            opcode: Opcode::STA,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x86, Instruction{
            opcode: Opcode::STX,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x8c, Instruction{
            opcode: Opcode::STY,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x8d, Instruction{
            opcode: Opcode::STA,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x8e, Instruction{
            opcode: Opcode::STX,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x90, Instruction{
            opcode: Opcode::BCC,
            mode: AddressingMode::Relative,
            steps: 2
        });
        m.insert(0x91, Instruction{
            opcode: Opcode::STA,
            mode: AddressingMode::IndirectY,
            steps: 6
        });
        m.insert(0x94, Instruction{
            opcode: Opcode::STY,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x95, Instruction{
            opcode: Opcode::STA,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x96, Instruction{
            opcode: Opcode::STX,
            mode: AddressingMode::ZeroPageY,
            steps: 4
        });
        m.insert(0x99, Instruction{
            opcode: Opcode::STA,
            mode: AddressingMode::AbsoluteY,
            steps: 5
        });
        m.insert(0x9d, Instruction{
            opcode: Opcode::STA,
            mode: AddressingMode::AbsoluteX,
            steps: 5
        });
        m.insert(0xa0, Instruction{
            opcode: Opcode::LDY,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xa1, Instruction{
            opcode: Opcode::LDA,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0xa2, Instruction{
            opcode: Opcode::LDX,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xa4, Instruction{
            opcode: Opcode::LDY,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xa5, Instruction{
            opcode: Opcode::LDA,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xa6, Instruction{
            opcode: Opcode::LDX,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xa9, Instruction{
            opcode: Opcode::LDA,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xac, Instruction{
            opcode: Opcode::LDY,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xad, Instruction{
            opcode: Opcode::LDA,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xae, Instruction{
            opcode: Opcode::LDX,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xb0, Instruction{
            opcode: Opcode::BCS,
            mode: AddressingMode::Relative,
            steps: 2
        });
        m.insert(0xb1, Instruction{
            opcode: Opcode::LDA,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0xb4, Instruction{
            opcode: Opcode::LDY,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0xb5, Instruction{
            opcode: Opcode::LDA,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0xb6, Instruction{
            opcode: Opcode::LDX,
            mode: AddressingMode::ZeroPageY,
            steps: 4
        });
        m.insert(0xb9, Instruction{
            opcode: Opcode::LDA,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0xbc, Instruction{
            opcode: Opcode::LDY,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0xbd, Instruction{
            opcode: Opcode::LDA,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0xbe, Instruction{
            opcode: Opcode::LDX,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0xc0, Instruction{
            opcode: Opcode::CPY,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xc1, Instruction{
            opcode: Opcode::CMP,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0xc4, Instruction{
            opcode: Opcode::CPY,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xc5, Instruction{
            opcode: Opcode::CMP,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xc6, Instruction{
            opcode: Opcode::DEC,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0xc9, Instruction{
            opcode: Opcode::CMP,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xcc, Instruction{
            opcode: Opcode::CPY,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xcd, Instruction{
            opcode: Opcode::CMP,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xce, Instruction{
            opcode: Opcode::DEC,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0xd0, Instruction{
            opcode: Opcode::BNE,
            mode: AddressingMode::Relative,
            steps: 2
        });
        m.insert(0xd1, Instruction{
            opcode: Opcode::CMP,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0xd5, Instruction{
            opcode: Opcode::CMP,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0xd6, Instruction{
            opcode: Opcode::DEC,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0xd9, Instruction{
            opcode: Opcode::CMP,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0xdd, Instruction{
            opcode: Opcode::CMP,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0xde, Instruction{
            opcode: Opcode::DEC,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0xe0, Instruction{
            opcode: Opcode::CPX,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xe1, Instruction{
            opcode: Opcode::SBC,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0xe4, Instruction{
            opcode: Opcode::CPX,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xe5, Instruction{
            opcode: Opcode::SBC,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xe6, Instruction{
            opcode: Opcode::INC,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0xe9, Instruction{
            opcode: Opcode::SBC,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xea, Instruction{
            opcode: Opcode::NOP,
            mode: AddressingMode::Implied,
            steps: 2
        });
        m.insert(0xec, Instruction{
            opcode: Opcode::CPX,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xed, Instruction{
            opcode: Opcode::SBC,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xee, Instruction{
            opcode: Opcode::INC,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0xf0, Instruction{
            opcode: Opcode::BEQ,
            mode: AddressingMode::Relative,
            steps: 2
        });
        m.insert(0xf1, Instruction{
            opcode: Opcode::SBC,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0xf5, Instruction{
            opcode: Opcode::SBC,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0xf6, Instruction{
            opcode: Opcode::INC,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0xf9, Instruction{
            opcode: Opcode::SBC,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0xfd, Instruction{
            opcode: Opcode::SBC,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0xfe, Instruction{
            opcode: Opcode::INC,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m
    };
}
