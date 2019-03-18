/// Define instruction codes

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub mode: AddressingMode,
    pub cycles: u8,
}

impl Instruction {
    pub fn from_code(code: u8) -> Instruction {
        lookup_instruction(code)
    }
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

fn lookup_instruction(code: u8) -> Instruction {
    match code {
        0x00 => Instruction {
            opcode: Opcode::BRK,
            mode: AddressingMode::Implied,
            cycles: 7,
        },
        0x01 => Instruction {
            opcode: Opcode::ORA,
            mode: AddressingMode::IndirectX,
            cycles: 6,
        },
        0x05 => Instruction {
            opcode: Opcode::ORA,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0x06 => Instruction {
            opcode: Opcode::ASL,
            mode: AddressingMode::ZeroPage,
            cycles: 5,
        },
        0x08 => Instruction {
            opcode: Opcode::PHP,
            mode: AddressingMode::Implied,
            cycles: 3,
        },
        0x09 => Instruction {
            opcode: Opcode::ORA,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0x0a => Instruction {
            opcode: Opcode::ASL,
            mode: AddressingMode::Accumulator,
            cycles: 2,
        },
        0x0d => Instruction {
            opcode: Opcode::ORA,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0x0e => Instruction {
            opcode: Opcode::ASL,
            mode: AddressingMode::Absolute,
            cycles: 6,
        },
        0x10 => Instruction {
            opcode: Opcode::BPL,
            mode: AddressingMode::Relative,
            cycles: 2,
        },
        0x11 => Instruction {
            opcode: Opcode::ORA,
            mode: AddressingMode::IndirectY,
            cycles: 5,
        },
        0x15 => Instruction {
            opcode: Opcode::ORA,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0x16 => Instruction {
            opcode: Opcode::ASL,
            mode: AddressingMode::ZeroPageX,
            cycles: 6,
        },
        0x18 => Instruction {
            opcode: Opcode::CLC,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0x19 => Instruction {
            opcode: Opcode::ORA,
            mode: AddressingMode::AbsoluteY,
            cycles: 4,
        },
        0x1d => Instruction {
            opcode: Opcode::ORA,
            mode: AddressingMode::AbsoluteX,
            cycles: 4,
        },
        0x1e => Instruction {
            opcode: Opcode::ASL,
            mode: AddressingMode::AbsoluteX,
            cycles: 7,
        },
        0x20 => Instruction {
            opcode: Opcode::JSR,
            mode: AddressingMode::Absolute,
            cycles: 6,
        },
        0x21 => Instruction {
            opcode: Opcode::AND,
            mode: AddressingMode::IndirectX,
            cycles: 6,
        },
        0x24 => Instruction {
            opcode: Opcode::BIT,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0x25 => Instruction {
            opcode: Opcode::AND,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0x26 => Instruction {
            opcode: Opcode::ROL,
            mode: AddressingMode::ZeroPage,
            cycles: 5,
        },
        0x28 => Instruction {
            opcode: Opcode::PLP,
            mode: AddressingMode::Implied,
            cycles: 4,
        },
        0x29 => Instruction {
            opcode: Opcode::AND,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0x2a => Instruction {
            opcode: Opcode::ROL,
            mode: AddressingMode::Accumulator,
            cycles: 2,
        },
        0x2c => Instruction {
            opcode: Opcode::BIT,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0x2d => Instruction {
            opcode: Opcode::AND,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0x2e => Instruction {
            opcode: Opcode::ROL,
            mode: AddressingMode::Absolute,
            cycles: 6,
        },
        0x30 => Instruction {
            opcode: Opcode::BMI,
            mode: AddressingMode::Relative,
            cycles: 2,
        },
        0x31 => Instruction {
            opcode: Opcode::AND,
            mode: AddressingMode::IndirectY,
            cycles: 5,
        },
        0x35 => Instruction {
            opcode: Opcode::AND,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0x36 => Instruction {
            opcode: Opcode::ROL,
            mode: AddressingMode::ZeroPageX,
            cycles: 6,
        },
        0x38 => Instruction {
            opcode: Opcode::SEC,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0x39 => Instruction {
            opcode: Opcode::AND,
            mode: AddressingMode::AbsoluteY,
            cycles: 4,
        },
        0x3d => Instruction {
            opcode: Opcode::AND,
            mode: AddressingMode::AbsoluteX,
            cycles: 4,
        },
        0x3e => Instruction {
            opcode: Opcode::ROL,
            mode: AddressingMode::AbsoluteX,
            cycles: 7,
        },
        0x40 => Instruction {
            opcode: Opcode::RTI,
            mode: AddressingMode::Implied,
            cycles: 6,
        },
        0x41 => Instruction {
            opcode: Opcode::EOR,
            mode: AddressingMode::IndirectX,
            cycles: 6,
        },
        0x45 => Instruction {
            opcode: Opcode::EOR,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0x46 => Instruction {
            opcode: Opcode::LSR,
            mode: AddressingMode::ZeroPage,
            cycles: 5,
        },
        0x48 => Instruction {
            opcode: Opcode::PHA,
            mode: AddressingMode::Implied,
            cycles: 3,
        },
        0x49 => Instruction {
            opcode: Opcode::EOR,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0x4a => Instruction {
            opcode: Opcode::LSR,
            mode: AddressingMode::Accumulator,
            cycles: 2,
        },
        0x4c => Instruction {
            opcode: Opcode::JMP,
            mode: AddressingMode::Absolute,
            cycles: 3,
        },
        0x4d => Instruction {
            opcode: Opcode::EOR,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0x4e => Instruction {
            opcode: Opcode::LSR,
            mode: AddressingMode::Absolute,
            cycles: 6,
        },
        0x50 => Instruction {
            opcode: Opcode::BVC,
            mode: AddressingMode::Relative,
            cycles: 2,
        },
        0x51 => Instruction {
            opcode: Opcode::EOR,
            mode: AddressingMode::IndirectY,
            cycles: 5,
        },
        0x55 => Instruction {
            opcode: Opcode::EOR,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0x56 => Instruction {
            opcode: Opcode::LSR,
            mode: AddressingMode::ZeroPageX,
            cycles: 6,
        },
        0x58 => Instruction {
            opcode: Opcode::CLI,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0x59 => Instruction {
            opcode: Opcode::EOR,
            mode: AddressingMode::AbsoluteY,
            cycles: 4,
        },
        0x5d => Instruction {
            opcode: Opcode::EOR,
            mode: AddressingMode::AbsoluteX,
            cycles: 4,
        },
        0x5e => Instruction {
            opcode: Opcode::LSR,
            mode: AddressingMode::AbsoluteX,
            cycles: 7,
        },
        0x60 => Instruction {
            opcode: Opcode::RTS,
            mode: AddressingMode::Implied,
            cycles: 6,
        },
        0x61 => Instruction {
            opcode: Opcode::ADC,
            mode: AddressingMode::IndirectX,
            cycles: 6,
        },
        0x65 => Instruction {
            opcode: Opcode::ADC,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0x66 => Instruction {
            opcode: Opcode::ROR,
            mode: AddressingMode::ZeroPage,
            cycles: 5,
        },
        0x68 => Instruction {
            opcode: Opcode::PLA,
            mode: AddressingMode::Implied,
            cycles: 4,
        },
        0x69 => Instruction {
            opcode: Opcode::ADC,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0x6a => Instruction {
            opcode: Opcode::ROR,
            mode: AddressingMode::Accumulator,
            cycles: 2,
        },
        0x6c => Instruction {
            opcode: Opcode::JMP,
            mode: AddressingMode::Indirect,
            cycles: 5,
        },
        0x6d => Instruction {
            opcode: Opcode::ADC,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0x6e => Instruction {
            opcode: Opcode::ROR,
            mode: AddressingMode::Absolute,
            cycles: 6,
        },
        0x70 => Instruction {
            opcode: Opcode::BVS,
            mode: AddressingMode::Relative,
            cycles: 2,
        },
        0x71 => Instruction {
            opcode: Opcode::ADC,
            mode: AddressingMode::IndirectY,
            cycles: 5,
        },
        0x75 => Instruction {
            opcode: Opcode::ADC,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0x76 => Instruction {
            opcode: Opcode::ROR,
            mode: AddressingMode::ZeroPageX,
            cycles: 6,
        },
        0x78 => Instruction {
            opcode: Opcode::SEI,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0x79 => Instruction {
            opcode: Opcode::ADC,
            mode: AddressingMode::AbsoluteY,
            cycles: 4,
        },
        0x7d => Instruction {
            opcode: Opcode::ADC,
            mode: AddressingMode::AbsoluteX,
            cycles: 4,
        },
        0x7e => Instruction {
            opcode: Opcode::ROR,
            mode: AddressingMode::AbsoluteX,
            cycles: 7,
        },
        0x81 => Instruction {
            opcode: Opcode::STA,
            mode: AddressingMode::IndirectX,
            cycles: 6,
        },
        0x84 => Instruction {
            opcode: Opcode::STY,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0x85 => Instruction {
            opcode: Opcode::STA,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0x86 => Instruction {
            opcode: Opcode::STX,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0x88 => Instruction {
            opcode: Opcode::DEY,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0x8a => Instruction {
            opcode: Opcode::TXA,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0x8c => Instruction {
            opcode: Opcode::STY,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0x8d => Instruction {
            opcode: Opcode::STA,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0x8e => Instruction {
            opcode: Opcode::STX,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0x90 => Instruction {
            opcode: Opcode::BCC,
            mode: AddressingMode::Relative,
            cycles: 2,
        },
        0x91 => Instruction {
            opcode: Opcode::STA,
            mode: AddressingMode::IndirectY,
            cycles: 6,
        },
        0x94 => Instruction {
            opcode: Opcode::STY,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0x95 => Instruction {
            opcode: Opcode::STA,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0x96 => Instruction {
            opcode: Opcode::STX,
            mode: AddressingMode::ZeroPageY,
            cycles: 4,
        },
        0x98 => Instruction {
            opcode: Opcode::TYA,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0x99 => Instruction {
            opcode: Opcode::STA,
            mode: AddressingMode::AbsoluteY,
            cycles: 5,
        },
        0x9a => Instruction {
            opcode: Opcode::TXS,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0x9d => Instruction {
            opcode: Opcode::STA,
            mode: AddressingMode::AbsoluteX,
            cycles: 5,
        },
        0xa0 => Instruction {
            opcode: Opcode::LDY,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0xa1 => Instruction {
            opcode: Opcode::LDA,
            mode: AddressingMode::IndirectX,
            cycles: 6,
        },
        0xa2 => Instruction {
            opcode: Opcode::LDX,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0xa4 => Instruction {
            opcode: Opcode::LDY,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0xa5 => Instruction {
            opcode: Opcode::LDA,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0xa6 => Instruction {
            opcode: Opcode::LDX,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0xa8 => Instruction {
            opcode: Opcode::TAY,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xa9 => Instruction {
            opcode: Opcode::LDA,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0xaa => Instruction {
            opcode: Opcode::TAX,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xac => Instruction {
            opcode: Opcode::LDY,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0xad => Instruction {
            opcode: Opcode::LDA,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0xae => Instruction {
            opcode: Opcode::LDX,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0xb0 => Instruction {
            opcode: Opcode::BCS,
            mode: AddressingMode::Relative,
            cycles: 2,
        },
        0xb1 => Instruction {
            opcode: Opcode::LDA,
            mode: AddressingMode::IndirectY,
            cycles: 5,
        },
        0xb4 => Instruction {
            opcode: Opcode::LDY,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0xb5 => Instruction {
            opcode: Opcode::LDA,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0xb6 => Instruction {
            opcode: Opcode::LDX,
            mode: AddressingMode::ZeroPageY,
            cycles: 4,
        },
        0xb8 => Instruction {
            opcode: Opcode::CLV,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xb9 => Instruction {
            opcode: Opcode::LDA,
            mode: AddressingMode::AbsoluteY,
            cycles: 4,
        },
        0xba => Instruction {
            opcode: Opcode::TSX,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xbc => Instruction {
            opcode: Opcode::LDY,
            mode: AddressingMode::AbsoluteX,
            cycles: 4,
        },
        0xbd => Instruction {
            opcode: Opcode::LDA,
            mode: AddressingMode::AbsoluteX,
            cycles: 4,
        },
        0xbe => Instruction {
            opcode: Opcode::LDX,
            mode: AddressingMode::AbsoluteY,
            cycles: 4,
        },
        0xc0 => Instruction {
            opcode: Opcode::CPY,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0xc1 => Instruction {
            opcode: Opcode::CMP,
            mode: AddressingMode::IndirectX,
            cycles: 6,
        },
        0xc4 => Instruction {
            opcode: Opcode::CPY,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0xc5 => Instruction {
            opcode: Opcode::CMP,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0xc6 => Instruction {
            opcode: Opcode::DEC,
            mode: AddressingMode::ZeroPage,
            cycles: 5,
        },
        0xc8 => Instruction {
            opcode: Opcode::INY,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xc9 => Instruction {
            opcode: Opcode::CMP,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0xca => Instruction {
            opcode: Opcode::DEX,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xcc => Instruction {
            opcode: Opcode::CPY,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0xcd => Instruction {
            opcode: Opcode::CMP,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0xce => Instruction {
            opcode: Opcode::DEC,
            mode: AddressingMode::Absolute,
            cycles: 6,
        },
        0xd0 => Instruction {
            opcode: Opcode::BNE,
            mode: AddressingMode::Relative,
            cycles: 2,
        },
        0xd1 => Instruction {
            opcode: Opcode::CMP,
            mode: AddressingMode::IndirectY,
            cycles: 5,
        },
        0xd5 => Instruction {
            opcode: Opcode::CMP,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0xd6 => Instruction {
            opcode: Opcode::DEC,
            mode: AddressingMode::ZeroPageX,
            cycles: 6,
        },
        0xd8 => Instruction {
            opcode: Opcode::CLD,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xd9 => Instruction {
            opcode: Opcode::CMP,
            mode: AddressingMode::AbsoluteY,
            cycles: 4,
        },
        0xdd => Instruction {
            opcode: Opcode::CMP,
            mode: AddressingMode::AbsoluteX,
            cycles: 4,
        },
        0xde => Instruction {
            opcode: Opcode::DEC,
            mode: AddressingMode::AbsoluteX,
            cycles: 7,
        },
        0xe0 => Instruction {
            opcode: Opcode::CPX,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0xe1 => Instruction {
            opcode: Opcode::SBC,
            mode: AddressingMode::IndirectX,
            cycles: 6,
        },
        0xe4 => Instruction {
            opcode: Opcode::CPX,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0xe5 => Instruction {
            opcode: Opcode::SBC,
            mode: AddressingMode::ZeroPage,
            cycles: 3,
        },
        0xe6 => Instruction {
            opcode: Opcode::INC,
            mode: AddressingMode::ZeroPage,
            cycles: 5,
        },
        0xe8 => Instruction {
            opcode: Opcode::INX,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xe9 => Instruction {
            opcode: Opcode::SBC,
            mode: AddressingMode::Immediate,
            cycles: 2,
        },
        0xea => Instruction {
            opcode: Opcode::NOP,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xec => Instruction {
            opcode: Opcode::CPX,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0xed => Instruction {
            opcode: Opcode::SBC,
            mode: AddressingMode::Absolute,
            cycles: 4,
        },
        0xee => Instruction {
            opcode: Opcode::INC,
            mode: AddressingMode::Absolute,
            cycles: 6,
        },
        0xf0 => Instruction {
            opcode: Opcode::BEQ,
            mode: AddressingMode::Relative,
            cycles: 2,
        },
        0xf1 => Instruction {
            opcode: Opcode::SBC,
            mode: AddressingMode::IndirectY,
            cycles: 5,
        },
        0xf5 => Instruction {
            opcode: Opcode::SBC,
            mode: AddressingMode::ZeroPageX,
            cycles: 4,
        },
        0xf6 => Instruction {
            opcode: Opcode::INC,
            mode: AddressingMode::ZeroPageX,
            cycles: 6,
        },
        0xf8 => Instruction {
            opcode: Opcode::SED,
            mode: AddressingMode::Implied,
            cycles: 2,
        },
        0xf9 => Instruction {
            opcode: Opcode::SBC,
            mode: AddressingMode::AbsoluteY,
            cycles: 4,
        },
        0xfd => Instruction {
            opcode: Opcode::SBC,
            mode: AddressingMode::AbsoluteX,
            cycles: 4,
        },
        0xfe => Instruction {
            opcode: Opcode::INC,
            mode: AddressingMode::AbsoluteX,
            cycles: 7,
        },
        _ => panic!("invalid instruction code {}", code),
    }
}
