use memory::Memory;
use super::register::Register;
use super::instructions::{lookup_instruction, AddressingMode, Opcode, Instruction};

#[derive(Debug, Copy, Clone)]
enum Operand {
    None,
    Immediate(u8),
    Accumulator,
    Memory(u16),
}

pub struct Cpu<M: Memory> {
    pub reg: Register,
    mem: M,

    remaining_clocks: u8,
}

impl<M: Memory> Cpu<M> {
    pub fn clock(&mut self) {
        if self.remaining_clocks > 0 {
            self.remaining_clocks -= 1;
        } else {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let inst = self.fetch_instrucion();
        let addr = self.fetch_operand(inst.mode);
        self.instruction(inst.opcode, addr);
        self.remaining_clocks = inst.steps-1;
    }

    fn set_zero_and_negative_flags(&mut self, val: u8) {
        self.reg.set_zero_flag(val == 0);
        self.reg.set_negative_flag((val & 0x80) != 0);
    }

    fn load_inst(&self, addr: Operand) -> u8 {
        match addr {
            Operand::None => unreachable!(),
            Operand::Immediate(val) => val,
            Operand::Accumulator => self.reg.A,
            Operand::Memory(addr) => self.mem.load(addr),
        }
    }

    fn write_inst(&mut self, addr: Operand, val: u8) {
        match addr {
            Operand::None => unreachable!(),
            Operand::Immediate(_) => unreachable!(),
            Operand::Accumulator => self.reg.A = val,
            Operand::Memory(addr) => self.mem.store(addr, val),
        }
    }

    fn jump_inst(&mut self, addr: Operand) {
        match addr {
            Operand::Immediate(val) => {
                let pc = self.reg.PC + val as u16;
                self.jump(pc);
            },
            Operand::Memory(addr) => self.jump(addr),
            _ => unreachable!(),
        }
    }

    fn comp_inst(&mut self, x: u8, m: u8) {
        self.reg.set_carry_flag(x >= m);
        self.reg.set_zero_flag(x == m);
        self.reg.set_negative_flag(x < m);
    }

    fn jump(&mut self, addr: u16) {
        self.reg.PC = addr;
    }

    fn push_stack(&mut self, val: u8) {
        self.reg.S -= 1;
        self.mem.store((self.reg.S+1) as u16, val);
    }

    fn push_stack_w(&mut self, val: u16) {
        self.push_stack((val & 0xff) as u8);
        self.push_stack((val >> 8) as u8);
    }

    fn pop_stack(&mut self) -> u8 {
        self.reg.S += 1;
        self.mem.load(self.reg.S as u16)
    }

    fn pop_stack_w(&mut self) -> u16 {
        let high = self.pop_stack();
        let low = self.pop_stack();
        low as u16 | ((high as u16) << 8)
    }

    fn fetch_instrucion(&mut self) -> Instruction {
        let code = self.mem.load(self.reg.PC);
        self.reg.PC += 1;
        lookup_instruction(code)
    }

    fn fetch_operand(&mut self, mode: AddressingMode) -> Operand {
        self.reg.PC += match mode {
            AddressingMode::Implied|AddressingMode::Accumulator => 0,
            AddressingMode::Absolute|AddressingMode::AbsoluteX|AddressingMode::AbsoluteY|AddressingMode::Indirect => 2,
            _ => 1
        };
        let im8 = || self.mem.load(self.reg.PC-1);
        let im16 = || self.mem.load_w(self.reg.PC-2);
        match mode {
            AddressingMode::Implied => Operand::None,
            AddressingMode::Accumulator => Operand::Accumulator,
            AddressingMode::Immediate => Operand::Immediate(im8()),
            AddressingMode::ZeroPage => Operand::Memory(im8() as u16),
            AddressingMode::ZeroPageX => Operand::Memory((im8() + self.reg.X) as u16),
            AddressingMode::ZeroPageY => Operand::Memory((im8() + self.reg.Y) as u16),
            AddressingMode::Absolute => Operand::Memory(im16()),
            AddressingMode::AbsoluteX => Operand::Memory(im16() + self.reg.X as u16),
            AddressingMode::AbsoluteY => Operand::Memory(im16() + self.reg.Y as u16),
            AddressingMode::Indirect => Operand::Memory(self.mem.load(im16()) as u16),
            AddressingMode::IndirectX => Operand::Memory(self.mem.load_w((im8() + self.reg.X) as u16)),
            AddressingMode::IndirectY => Operand::Memory(self.mem.load_w((im8() as u16)) + self.reg.Y as u16),
            AddressingMode::Relative => Operand::Immediate(im8()),
        }
    }

    fn instruction(&mut self, op: Opcode, addr: Operand) {
        match op {
            Opcode::LDA => {
                let val = self.load_inst(addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::LDX => {
                let val = self.load_inst(addr);
                self.reg.X = val; 
                self.set_zero_and_negative_flags(val);
            },
            Opcode::LDY => {
                let val = self.load_inst(addr);
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::STA => {
                let a = self.reg.A;
                self.write_inst(addr, a);
            },
            Opcode::STX => {
                let x = self.reg.X;
                self.write_inst(addr, x);
            },
            Opcode::STY => {
                let y = self.reg.Y;
                self.write_inst(addr, y);
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
                self.push_stack(val);
            },
            Opcode::PHP => {
                let val = self.reg.P;
                self.push_stack(val);
            },
            Opcode::PLA => {
                let val = self.pop_stack();
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::PLP => {
                let val = self.pop_stack();
                self.reg.P = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::AND => {
                let val = self.reg.A & self.load_inst(addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::EOR => {
                let val = self.reg.A ^ self.load_inst(addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::ORA => {
                let val = self.reg.A | self.load_inst(addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::BIT => {
                let val = self.reg.A | self.load_inst(addr);
                self.reg.set_overflow_flag(val & 0x40 != 0);
                self.reg.set_negative_flag(val & 0x80 != 0);
            },
            Opcode::ADC => {
                // TODO: overflow and carry flag
                let val = self.reg.A + self.load_inst(addr) + self.reg.carry_flag() as u8;
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::SBC => {
                // TODO: overflow and carry flag
                let val = self.reg.A - self.load_inst(addr) - (1 - self.reg.carry_flag() as u8);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            },
            Opcode::CMP => {
                let a = self.reg.A;
                let m = self.load_inst(addr);
                self.comp_inst(a, m);
            },
            Opcode::CPX => {
                let x = self.reg.X;
                let m = self.load_inst(addr);
                self.comp_inst(x, m);
            },
            Opcode::CPY => {
                let y = self.reg.Y;
                let m = self.load_inst(addr);
                self.comp_inst(y, m);
            },
            Opcode::INC => {
                let val = self.load_inst(addr) + 1;
                self.write_inst(addr, val);
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
                let val = self.load_inst(addr) - 1;
                self.write_inst(addr, val);
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
                let val = self.load_inst(addr);
                self.reg.set_carry_flag(val & 0x80 != 0);
                self.reg.set_negative_flag(val & 0x40 != 0);
                self.reg.set_zero_flag(val == 0);
                self.write_inst(addr, (val & 0x7f) << 1);
            },
            Opcode::LSR => {
                let val = self.load_inst(addr);
                self.reg.set_carry_flag(val & 0x01 != 0);
                self.reg.set_negative_flag(false);
                self.reg.set_zero_flag(val == 0);
                self.write_inst(addr, val >> 1);
            },
            Opcode::ROL => {
                let val = self.load_inst(addr);
                self.reg.set_carry_flag(val & 0x80 != 0);
                self.reg.set_negative_flag(val & 0x40 != 0);
                self.reg.set_zero_flag(val == 0);
                self.write_inst(addr, ((val & 0x7f) << 1) | ((val & 0x80) >> 7));
            },
            Opcode::ROR => {
                let val = self.load_inst(addr);
                self.reg.set_carry_flag(val & 0x01 != 0);
                self.reg.set_negative_flag(val & 0x01 != 0);
                self.reg.set_zero_flag(val == 0);
                self.write_inst(addr, (val >> 1) | ((val & 0x01) << 7));
            },
            Opcode::JMP => {
                self.jump_inst(addr);
            },
            Opcode::JSR => {
                let pc = self.reg.PC-1;
                self.push_stack_w(pc);
                self.jump_inst(addr);
            }
            Opcode::RTS => {
                let addr = self.pop_stack_w();
                self.jump(addr+1);
            }
            Opcode::BCC => {
                if !self.reg.carry_flag() {
                    self.jump_inst(addr);
                }
            },
            Opcode::BCS => {
                if self.reg.carry_flag() {
                    self.jump_inst(addr);
                }
            },
            Opcode::BEQ => {
                if self.reg.zero_flag() {
                    self.jump_inst(addr);
                }
            },
            Opcode::BMI => {
                if self.reg.negative_flag() {
                    self.jump_inst(addr);
                }
            },
            Opcode::BNE => {
                if !self.reg.zero_flag() {
                    self.jump_inst(addr);
                }
            },
            Opcode::BPL => {
                if !self.reg.negative_flag() {
                    self.jump_inst(addr);
                }
            },
            Opcode::BVC => {
                if !self.reg.overflow_flag() {
                    self.jump_inst(addr);
                }
            },
            Opcode::BVS => {
                if self.reg.overflow_flag() {
                    self.jump_inst(addr);
                }
            },
            Opcode::CLC => {
                self.reg.set_carry_flag(false);
            },
            Opcode::CLD => {
                // unimplemented in NES
            },
            Opcode::CLI => {
                unimplemented!()
            },
            Opcode::CLV => {
                self.reg.set_overflow_flag(false);
            },
            Opcode::SEC => {
                self.reg.set_carry_flag(false);
            },
            Opcode::SED => {
                // unimplemented in NES
            },
            Opcode::SEI => {
                unimplemented!()
            },
            Opcode::BRK => {
                unimplemented!()
            },
            Opcode::NOP => {
                // no operation
            },
            Opcode::RTI => {
                unimplemented!()
            }
        }
    }
}