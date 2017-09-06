use std::collections::HashMap;

pub fn get_instruction(x: u8) -> Instruction {
    match INSTRUCTIONS.get(&x) {
        Some(inst) => *inst,
        None => panic!("invalid instruction code")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub op: Opcode,
    pub mode: AddressingMode,
    pub steps: u8
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
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    ADC, AND, ASL, BCC, BCS,
    BEQ, BIT, BMI, BNE, BPL,
    BRK, BVC, BVS, CLC, CLD,
    CLI, CLV, CMP, CPX, CPY,
    DEC, DEX, DEY, EOR, INC,
    INX, INY, JMP, JSR, LDA,
    LDX, LDY, LSR, NOP, ORA,
    PHA, PHP, PLA, PLP, ROL,
    ROR, RTI, RTS, SBC, SEC,
    SED, SEI, STA, STX, STY,
    TAX, TAY, TSX, TXA, TXS,
    TYA,
}

lazy_static! {
    static ref INSTRUCTIONS: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0x00, Instruction{
            op: Opcode::BRK,
            mode: AddressingMode::Implied,
            steps: 7
        });
        m.insert(0x01, Instruction{
            op: Opcode::ORA,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x05, Instruction{
            op: Opcode::ORA,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x06, Instruction{
            op: Opcode::ASL,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0x09, Instruction{
            op: Opcode::ORA,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0x0a, Instruction{
            op: Opcode::ASL,
            mode: AddressingMode::Accumulator,
            steps: 2
        });
        m.insert(0x0d, Instruction{
            op: Opcode::ORA,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x0e, Instruction{
            op: Opcode::ASL,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x11, Instruction{
            op: Opcode::ORA,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0x15, Instruction{
            op: Opcode::ORA,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x16, Instruction{
            op: Opcode::ASL,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0x19, Instruction{
            op: Opcode::ORA,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0x1d, Instruction{
            op: Opcode::ORA,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0x1e, Instruction{
            op: Opcode::ASL,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0x20, Instruction{
            op: Opcode::JSR,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x21, Instruction{
            op: Opcode::AND,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x24, Instruction{
            op: Opcode::BIT,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x25, Instruction{
            op: Opcode::AND,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x26, Instruction{
            op: Opcode::ROL,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0x29, Instruction{
            op: Opcode::AND,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0x2a, Instruction{
            op: Opcode::ROL,
            mode: AddressingMode::Accumulator,
            steps: 2
        });
        m.insert(0x2c, Instruction{
            op: Opcode::BIT,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x2d, Instruction{
            op: Opcode::AND,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x2e, Instruction{
            op: Opcode::ROL,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x31, Instruction{
            op: Opcode::AND,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0x35, Instruction{
            op: Opcode::AND,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x36, Instruction{
            op: Opcode::ROL,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0x39, Instruction{
            op: Opcode::AND,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0x3d, Instruction{
            op: Opcode::AND,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0x3e, Instruction{
            op: Opcode::ROL,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0x40, Instruction{
            op: Opcode::RTI,
            mode: AddressingMode::Implied,
            steps: 6
        });
        m.insert(0x41, Instruction{
            op: Opcode::EOR,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x45, Instruction{
            op: Opcode::EOR,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x46, Instruction{
            op: Opcode::LSR,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0x49, Instruction{
            op: Opcode::EOR,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0x4a, Instruction{
            op: Opcode::LSR,
            mode: AddressingMode::Accumulator,
            steps: 2
        });
        m.insert(0x4c, Instruction{
            op: Opcode::JMP,
            mode: AddressingMode::Absolute,
            steps: 3
        });
        m.insert(0x4d, Instruction{
            op: Opcode::EOR,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x4e, Instruction{
            op: Opcode::LSR,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x51, Instruction{
            op: Opcode::EOR,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0x55, Instruction{
            op: Opcode::EOR,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x56, Instruction{
            op: Opcode::LSR,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0x59, Instruction{
            op: Opcode::EOR,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0x5d, Instruction{
            op: Opcode::EOR,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0x5e, Instruction{
            op: Opcode::LSR,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0x60, Instruction{
            op: Opcode::RTS,
            mode: AddressingMode::Implied,
            steps: 6
        });
        m.insert(0x61, Instruction{
            op: Opcode::ADC,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x65, Instruction{
            op: Opcode::ADC,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x66, Instruction{
            op: Opcode::ROR,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0x69, Instruction{
            op: Opcode::ADC,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0x6a, Instruction{
            op: Opcode::ROR,
            mode: AddressingMode::Accumulator,
            steps: 2
        });
        m.insert(0x6c, Instruction{
            op: Opcode::JMP,
            mode: AddressingMode::Indirect,
            steps: 5
        });
        m.insert(0x6d, Instruction{
            op: Opcode::ADC,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x6e, Instruction{
            op: Opcode::ROR,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0x71, Instruction{
            op: Opcode::ADC,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0x75, Instruction{
            op: Opcode::ADC,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x76, Instruction{
            op: Opcode::ROR,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0x79, Instruction{
            op: Opcode::ADC,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0x7d, Instruction{
            op: Opcode::ADC,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0x7e, Instruction{
            op: Opcode::ROR,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0x81, Instruction{
            op: Opcode::STA,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0x84, Instruction{
            op: Opcode::STY,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x85, Instruction{
            op: Opcode::STA,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x86, Instruction{
            op: Opcode::STX,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0x8c, Instruction{
            op: Opcode::STY,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x8d, Instruction{
            op: Opcode::STA,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x8e, Instruction{
            op: Opcode::STX,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0x91, Instruction{
            op: Opcode::STA,
            mode: AddressingMode::IndirectY,
            steps: 6
        });
        m.insert(0x94, Instruction{
            op: Opcode::STY,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x95, Instruction{
            op: Opcode::STA,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0x96, Instruction{
            op: Opcode::STX,
            mode: AddressingMode::ZeroPageY,
            steps: 4
        });
        m.insert(0x99, Instruction{
            op: Opcode::STA,
            mode: AddressingMode::AbsoluteY,
            steps: 5
        });
        m.insert(0x9d, Instruction{
            op: Opcode::STA,
            mode: AddressingMode::AbsoluteX,
            steps: 5
        });
        m.insert(0xa0, Instruction{
            op: Opcode::LDY,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xa1, Instruction{
            op: Opcode::LDA,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0xa2, Instruction{
            op: Opcode::LDX,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xa4, Instruction{
            op: Opcode::LDY,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xa5, Instruction{
            op: Opcode::LDA,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xa6, Instruction{
            op: Opcode::LDX,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xa9, Instruction{
            op: Opcode::LDA,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xac, Instruction{
            op: Opcode::LDY,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xad, Instruction{
            op: Opcode::LDA,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xae, Instruction{
            op: Opcode::LDX,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xb1, Instruction{
            op: Opcode::LDA,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0xb4, Instruction{
            op: Opcode::LDY,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0xb5, Instruction{
            op: Opcode::LDA,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0xb6, Instruction{
            op: Opcode::LDX,
            mode: AddressingMode::ZeroPageY,
            steps: 4
        });
        m.insert(0xb9, Instruction{
            op: Opcode::LDA,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0xbc, Instruction{
            op: Opcode::LDY,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0xbd, Instruction{
            op: Opcode::LDA,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0xbe, Instruction{
            op: Opcode::LDX,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0xc0, Instruction{
            op: Opcode::CPY,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xc1, Instruction{
            op: Opcode::CMP,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0xc4, Instruction{
            op: Opcode::CPY,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xc5, Instruction{
            op: Opcode::CMP,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xc6, Instruction{
            op: Opcode::DEC,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0xc9, Instruction{
            op: Opcode::CMP,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xcc, Instruction{
            op: Opcode::CPY,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xcd, Instruction{
            op: Opcode::CMP,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xce, Instruction{
            op: Opcode::DEC,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0xd1, Instruction{
            op: Opcode::CMP,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0xd5, Instruction{
            op: Opcode::CMP,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0xd6, Instruction{
            op: Opcode::DEC,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0xd9, Instruction{
            op: Opcode::CMP,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0xdd, Instruction{
            op: Opcode::CMP,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0xde, Instruction{
            op: Opcode::DEC,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m.insert(0xe0, Instruction{
            op: Opcode::CPX,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xe1, Instruction{
            op: Opcode::SBC,
            mode: AddressingMode::IndirectX,
            steps: 6
        });
        m.insert(0xe4, Instruction{
            op: Opcode::CPX,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xe5, Instruction{
            op: Opcode::SBC,
            mode: AddressingMode::ZeroPage,
            steps: 3
        });
        m.insert(0xe6, Instruction{
            op: Opcode::INC,
            mode: AddressingMode::ZeroPage,
            steps: 5
        });
        m.insert(0xe9, Instruction{
            op: Opcode::SBC,
            mode: AddressingMode::Immediate,
            steps: 2
        });
        m.insert(0xea, Instruction{
            op: Opcode::NOP,
            mode: AddressingMode::Implied,
            steps: 2
        });
        m.insert(0xec, Instruction{
            op: Opcode::CPX,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xed, Instruction{
            op: Opcode::SBC,
            mode: AddressingMode::Absolute,
            steps: 4
        });
        m.insert(0xee, Instruction{
            op: Opcode::INC,
            mode: AddressingMode::Absolute,
            steps: 6
        });
        m.insert(0xf1, Instruction{
            op: Opcode::SBC,
            mode: AddressingMode::IndirectY,
            steps: 5
        });
        m.insert(0xf5, Instruction{
            op: Opcode::SBC,
            mode: AddressingMode::ZeroPageX,
            steps: 4
        });
        m.insert(0xf6, Instruction{
            op: Opcode::INC,
            mode: AddressingMode::ZeroPageX,
            steps: 6
        });
        m.insert(0xf9, Instruction{
            op: Opcode::SBC,
            mode: AddressingMode::AbsoluteY,
            steps: 4
        });
        m.insert(0xfd, Instruction{
            op: Opcode::SBC,
            mode: AddressingMode::AbsoluteX,
            steps: 4
        });
        m.insert(0xfe, Instruction{
            op: Opcode::INC,
            mode: AddressingMode::AbsoluteX,
            steps: 7
        });
        m
    };
}
