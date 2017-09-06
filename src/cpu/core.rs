use memory::Memory;
use super::register::Register;
use super::instructions::{get_instruction, AddressingMode, Opcode, Instruction};

#[derive(Debug, Copy, Clone)]
enum Addr {
    None,
    Immediate(u8),
    Accumulator,
    Memory(u16),
}

pub struct Cpu<M: Memory> {
    pub reg: Register,
    mem: M,
}

impl<M: Memory> Cpu<M> {
    pub fn step(&mut self) {
        let code: u8 = self.mem.read(self.reg.PC);
        let inst: Instruction = get_instruction(code);
        let addr = self.get_addr(inst.mode);
        self.exec(inst.op, addr);
    }

    fn read(&self, addr: Addr) -> u8 {
        match addr {
            Addr::None => unreachable!(),
            Addr::Immediate(val) => val,
            Addr::Accumulator => self.reg.A,
            Addr::Memory(addr) => self.mem.read(addr),
        }
    }

    fn read_dw(&self, addr: Addr) -> u16 {
        match addr {
            Addr::None => unreachable!(),
            Addr::Immediate(_) => unreachable!(),
            Addr::Accumulator => unreachable!(),
            Addr::Memory(addr) => (self.mem.read(addr) as u16) | ((self.mem.read(addr+1) as u16) << 8),
        }
    }

    fn write(&mut self, addr: Addr, val: u8) {
        match addr {
            Addr::None => unreachable!(),
            Addr::Immediate(_) => unreachable!(),
            Addr::Accumulator => self.reg.A = val,
            Addr::Memory(addr) => self.mem.write(addr, val),
        }
    }

    fn set_zero_and_negative_flags(&mut self, val: u8) {
        self.reg.set_zero_flag(val == 0);
        self.reg.set_negative_flag(val & 0x80 != 0);
    }

    fn get_addr(&self, mode: AddressingMode) -> Addr {
        let pc = self.reg.PC + 1;
        let im8 = || self.mem.read(pc) as u8;
        let im16 = || (self.mem.read(pc) as u16) | ((self.mem.read(pc+1) as u16) << 8);
        match mode {
            AddressingMode::Implied => Addr::None,
            AddressingMode::Accumulator => Addr::Accumulator,
            AddressingMode::Immediate => Addr::Immediate(im8()),
            AddressingMode::ZeroPage => Addr::Memory(im8() as u16),
            AddressingMode::ZeroPageX => Addr::Memory((im8() + self.reg.X) as u16),
            AddressingMode::ZeroPageY => Addr::Memory((im8() + self.reg.Y) as u16),
            AddressingMode::Absolute => Addr::Memory(im16()),
            AddressingMode::AbsoluteX => Addr::Memory(im16() + self.reg.X as u16),
            AddressingMode::AbsoluteY => Addr::Memory(im16() + self.reg.Y as u16),
            AddressingMode::Indirect => Addr::Memory(im16()),
            AddressingMode::IndirectX => Addr::Memory(self.read_dw(Addr::Memory((im8() + self.reg.X) as u16))),
            AddressingMode::IndirectY => Addr::Memory(self.read_dw(Addr::Memory(im8() as u16)) + self.reg.Y as u16)
        }
    }

    fn exec(&mut self, op: Opcode, addr: Addr) {
        match op {
            Opcode::LDA => {
                let val = self.read(addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::LDX => {
                let val = self.read(addr);
                self.reg.X = val; 
                self.set_zero_and_negative_flags(val);
            },
            Opcode::LDY => {
                let val = self.read(addr);
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::STX => {
                let x = self.reg.X;
                self.write(addr, x);
            },
            Opcode::STY => {
                let y = self.reg.Y;
                self.write(addr, y);
            },
            Opcode::TAX => {
                let val = self.reg.A;
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::TAY => {
                let val = self.reg.A;
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::TSX => {
                let val = self.reg.S;
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::TXA => {
                let val = self.reg.X;
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::TXS => {
                let val = self.reg.X;
                self.reg.S = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::TYA => {
                let val = self.reg.Y;
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::PHA => {
                let val = self.reg.A;
                self.mem.write(self.reg.S as u16, val);
                self.reg.S -= 1;
            },
            Opcode::PHP => {
                let val = self.reg.P;
                self.mem.write(self.reg.S as u16, val);
                self.reg.S -= 1;
            },
            Opcode::PLA => {
                self.reg.S += 1;
                let val = self.mem.read(self.reg.S as u16);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::PLP => {
                self.reg.S += 1;
                let val = self.mem.read(self.reg.S as u16);
                self.reg.P = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::AND => {
                let val = self.reg.A & self.read(addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::EOR => {
                let val = self.reg.A ^ self.read(addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::ORA => {
                let val = self.reg.A | self.read(addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::BIT => {
                let val = self.reg.A | self.read(addr);
                self.reg.set_overflow_flag(val & 0x40 != 0);
                self.reg.set_negative_flag(val & 0x80 != 0);
            },
            Opcode::ADC => {
                // TODO: overflow and carry flag
                let val = self.reg.A + self.read(addr) + self.reg.carry_flag() as u8;
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::SBC => {
                // TODO: overflow and carry flag
                let val = self.reg.A - self.read(addr) - (1 - self.reg.carry_flag() as u8);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::CMP => {
                let a = self.reg.A;
                let m = self.read(addr);
                self.reg.set_carry_flag(a >= m);
                self.reg.set_zero_flag(a == m);
                self.reg.set_negative_flag(a < m);
            },
            Opcode::CPX => {
                let x = self.reg.X;
                let m = self.read(addr);
                self.reg.set_carry_flag(x >= m);
                self.reg.set_zero_flag(x == m);
                self.reg.set_negative_flag(x < m);
            },
            Opcode::CPY => {
                let y = self.reg.Y;
                let m = self.read(addr);
                self.reg.set_carry_flag(y >= m);
                self.reg.set_zero_flag(y == m);
                self.reg.set_negative_flag(y < m);
            },
            Opcode::INC => {
                let val = self.read(addr) + 1;
                self.write(addr, val);
                self.set_zero_and_negative_flags(val);
            }
            Opcode::INX => {
                let val = self.reg.X + 1;
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::INY => {
                let val = self.reg.Y + 1;
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::DEC => {
                let val = self.read(addr) - 1;
                self.write(addr, val);
                self.set_zero_and_negative_flags(val);
            }
            Opcode::DEX => {
                let val = self.reg.X - 1;
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::DEY => {
                let val = self.reg.Y - 1;
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::ASL => {
                let val = self.read(addr);
                self.reg.set_carry_flag(val & 0x80 != 0);
                self.reg.set_negative_flag(val & 0x40 != 0);
                self.reg.set_zero_flag(val == 0);
                self.write(addr, (val & 0x7f) << 1);
            },
            Opcode::LSR => {
                let val = self.read(addr);
                self.reg.set_carry_flag(val & 0x01 != 0);
                self.reg.set_negative_flag(false);
                self.reg.set_zero_flag(val == 0);
                self.write(addr, val >> 1);
            },
            Opcode::ROL => {
                let val = self.read(addr);
                self.reg.set_carry_flag(val & 0x80 != 0);
                self.reg.set_negative_flag(val & 0x40 != 0);
                self.reg.set_zero_flag(val == 0);
                self.write(addr, ((val & 0x7f) << 1) | ((val & 0x80) >> 7));
            },
            Opcode::ROR => {
                let val = self.read(addr);
                self.reg.set_carry_flag(val & 0x01 != 0);
                self.reg.set_negative_flag(val & 0x01 != 0);
                self.reg.set_zero_flag(val == 0);
                self.write(addr, (val >> 1) | ((val & 0x01) << 7));
            },
            _ => unimplemented!()
        }
    }
}