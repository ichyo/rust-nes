use super::instructions::{AddressingMode, Instruction, Opcode};
use super::register::Register;
use crate::bus::Bus;
use log::{info, trace};

#[derive(Debug, Copy, Clone)]
enum Operand {
    None,
    Immediate(u8),
    Accumulator,
    Memory(u16),
}

#[derive(Default)]
/// Cpu model with register
pub struct Cpu {
    reg: Register,
}

impl Cpu {
    /// Create cpu model with initial register
    pub fn new() -> Self {
        Cpu {
            reg: Register::new(),
        }
    }
}

impl Cpu {
    /// Fetches and executes instruction.
    /// Returns the number of clocks
    pub fn exec(&mut self, bus: &mut Bus) -> u8 {
        let pc = self.reg.PC;
        let inst = self.fetch_instruction(bus);
        let addr = self.fetch_operand(bus, inst.mode);
        let inst_bytes = 1 + u16::from(inst.mode.operand_bytes());

        let code = (0..inst_bytes)
            .map(|d| bus.load(pc + d))
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<_>>()
            .join(" ");

        let addr_fmt = match addr {
            Operand::None => "".to_string(),
            Operand::Immediate(x) => format!("#${:02X}", x),
            Operand::Accumulator => "?".to_string(),
            Operand::Memory(x) if inst.mode.operand_bytes() == 1 => format!("${:02X}", x),
            Operand::Memory(x) => format!("${:04X}", x),
        };

        let addr_value_fmt = if let Operand::Memory(x) = addr {
            match inst.opcode {
                Opcode::JMP
                | Opcode::JSR
                | Opcode::BCS
                | Opcode::BCC
                | Opcode::BEQ
                | Opcode::BNE
                | Opcode::BVS
                | Opcode::BVC
                | Opcode::BPL
                | Opcode::BMI => "".to_string(),
                _ => format!(" = {:02X}", bus.load(x)),
            }
        } else {
            "".to_string()
        };

        let reg_fmt = format!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.reg.A,
            self.reg.X,
            self.reg.Y,
            self.reg.P.to_u8(),
            self.reg.S
        );

        trace!(
            "{:4X}  {:8}  {:?} {:26}  {}",
            pc,
            code,
            inst.opcode,
            addr_fmt + &addr_value_fmt,
            reg_fmt,
        );

        self.reg.PC += inst_bytes;
        self.execute_instruction(bus, inst.opcode, addr);

        inst.cycles
    }

    /// Reset state
    pub fn reset(&mut self, bus: &mut Bus) {
        self.reg = Register::new();
        self.reg.PC = bus.load_w(0xfffc);
    }

    /// NMI interrupts
    pub fn nmi(&mut self, bus: &mut Bus) {
        // TODO: this is for testing. implement correctly
        self.reg.PC = bus.load_w(0xfffa);
        info!("nmi loaded {}", self.reg.PC);
    }

    fn set_zero_and_negative_flags(&mut self, val: u8) {
        self.reg.P.set_zero_flag(val == 0);
        self.reg.P.set_negative_flag((val & 0x80) != 0);
    }

    fn load_inst(&mut self, bus: &mut Bus, addr: Operand) -> u8 {
        match addr {
            Operand::None => unreachable!(),
            Operand::Immediate(val) => val,
            Operand::Accumulator => self.reg.A,
            Operand::Memory(addr) => bus.load(addr),
        }
    }

    fn write_inst(&mut self, bus: &mut Bus, addr: Operand, val: u8) {
        match addr {
            Operand::None => unreachable!(),
            Operand::Immediate(_) => unreachable!(),
            Operand::Accumulator => self.reg.A = val,
            Operand::Memory(addr) => bus.store(addr, val),
        }
    }

    fn jump_inst(&mut self, addr: Operand) {
        match addr {
            /*
            Operand::Immediate(val) => {
                let pc = i32::from(self.reg.PC) + i32::from(val as i8);
                self.jump(pc as u16);
            }
            */
            Operand::Memory(addr) => self.jump(addr),
            _ => unreachable!(),
        }
    }

    fn comp_inst(&mut self, x: u8, m: u8) {
        let r = x.wrapping_sub(m);
        self.reg.P.set_carry_flag(x >= m);
        self.reg.P.set_zero_flag(x == m);
        self.reg.P.set_negative_flag((r & 0x80) != 0);
    }

    fn jump(&mut self, addr: u16) {
        self.reg.PC = addr;
    }

    fn push_stack(&mut self, bus: &mut Bus, val: u8) {
        self.reg.S -= 1;
        bus.store(u16::from(self.reg.S + 1) + 0x100, val);
    }

    fn push_stack_w(&mut self, bus: &mut Bus, val: u16) {
        self.push_stack(bus, (val >> 8) as u8);
        self.push_stack(bus, (val & 0xff) as u8);
    }

    fn pop_stack(&mut self, bus: &mut Bus) -> u8 {
        self.reg.S += 1;
        bus.load(u16::from(self.reg.S) + 0x100)
    }

    fn pop_stack_w(&mut self, bus: &mut Bus) -> u16 {
        let low = self.pop_stack(bus);
        let high = self.pop_stack(bus);
        u16::from(low) | (u16::from(high) << 8)
    }

    fn fetch_instruction(&self, bus: &mut Bus) -> Instruction {
        let code = bus.load(self.reg.PC);
        Instruction::from_code(code)
    }

    fn fetch_operand(&self, bus: &mut Bus, mode: AddressingMode) -> Operand {
        let addr = self.reg.PC + 1;
        match mode {
            AddressingMode::Implied => Operand::None,
            AddressingMode::Accumulator => Operand::Accumulator,
            AddressingMode::Immediate => {
                let value = bus.load(addr);
                Operand::Immediate(value)
            }
            AddressingMode::ZeroPage => {
                let value = bus.load(addr);
                Operand::Memory(u16::from(value))
            }
            AddressingMode::ZeroPageX => {
                let value = bus.load(addr);
                Operand::Memory(u16::from(value.wrapping_add(self.reg.X)))
            }
            AddressingMode::ZeroPageY => {
                let value = bus.load(addr);
                Operand::Memory(u16::from(value + self.reg.Y))
            }
            AddressingMode::Absolute => {
                let value = bus.load_w(addr);
                Operand::Memory(value)
            }
            AddressingMode::AbsoluteX => {
                let value = bus.load_w(addr);
                Operand::Memory(value + u16::from(self.reg.X))
            }
            AddressingMode::AbsoluteY => {
                let value = bus.load_w(addr);
                Operand::Memory(value + u16::from(self.reg.Y))
            }
            AddressingMode::Indirect => {
                let addr = bus.load_w(addr);
                let value = bus.load(addr);
                Operand::Memory(u16::from(value))
            }
            AddressingMode::IndirectX => {
                let addr = bus.load(addr) + self.reg.X;
                let value = bus.load_w(u16::from(addr));
                Operand::Memory(value)
            }
            AddressingMode::IndirectY => {
                let addr = bus.load(addr);
                let value = bus.load_w(u16::from(addr)) + u16::from(self.reg.Y);
                Operand::Memory(value)
            }
            AddressingMode::Relative => {
                let value = bus.load(addr);
                Operand::Memory(self.reg.PC + u16::from(value) + 2)
            }
        }
    }

    fn execute_instruction(&mut self, bus: &mut Bus, op: Opcode, addr: Operand) {
        match op {
            Opcode::LDA => {
                let val = self.load_inst(bus, addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::LDX => {
                let val = self.load_inst(bus, addr);
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::LDY => {
                let val = self.load_inst(bus, addr);
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::STA => {
                let a = self.reg.A;
                self.write_inst(bus, addr, a);
            }
            Opcode::STX => {
                let x = self.reg.X;
                self.write_inst(bus, addr, x);
            }
            Opcode::STY => {
                let y = self.reg.Y;
                self.write_inst(bus, addr, y);
            }
            Opcode::TAX => {
                let val = self.reg.A;
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::TAY => {
                let val = self.reg.A;
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::TSX => {
                let val = self.reg.S;
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::TXA => {
                let val = self.reg.X;
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::TXS => {
                let val = self.reg.X;
                self.reg.S = val;
            }
            Opcode::TYA => {
                let val = self.reg.Y;
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::PHA => {
                let val = self.reg.A;
                self.push_stack(bus, val);
            }
            Opcode::PHP => {
                let mut p = self.reg.P.clone();
                p.set_break_command(true);
                let val = p.to_u8();
                self.push_stack(bus, val);
            }
            Opcode::PLA => {
                let val = self.pop_stack(bus);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::PLP => {
                let val = self.pop_stack(bus);
                self.reg.P.set_u8(val);
                self.reg.P.set_break_command(false);
            }
            Opcode::AND => {
                let val = self.reg.A & self.load_inst(bus, addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::EOR => {
                let val = self.reg.A ^ self.load_inst(bus, addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::ORA => {
                let val = self.reg.A | self.load_inst(bus, addr);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::BIT => {
                let m = self.load_inst(bus, addr);
                let and = self.reg.A & m;
                self.reg.P.set_zero_flag(and == 0);
                self.reg.P.set_overflow_flag((m & 0x40) != 0);
                self.reg.P.set_negative_flag((m & 0x80) != 0);
            }
            Opcode::ADC => {
                let (x, y) = (self.reg.A, self.load_inst(bus, addr));
                let (val, overflow1) = x.overflowing_add(y);
                let (val, overflow2) = val.overflowing_add(self.reg.P.carry_flag() as u8);
                let carry = overflow1 || overflow2;
                let x_neg = (x & 0x80) != 0;
                let y_neg = (y & 0x80) != 0;
                let val_neg = (val & 0x80) != 0;
                let overflow = (x_neg && y_neg && !val_neg) || (!x_neg && !y_neg && val_neg);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
                self.reg.P.set_carry_flag(carry);
                self.reg.P.set_overflow_flag(overflow);
            }
            Opcode::SBC => {
                let (x, y) = (self.reg.A, self.load_inst(bus, addr));
                let (val, overflow1) = x.overflowing_sub(y);
                let (val, overflow2) = val.overflowing_sub(1 - self.reg.P.carry_flag() as u8);
                let carry = !(overflow1 || overflow2);
                let x_neg = (x & 0x80) != 0;
                let y_neg = (y & 0x80) != 0;
                let val_neg = (val & 0x80) != 0;
                let overflow = (x_neg && !y_neg && !val_neg) || (!x_neg && y_neg && val_neg);
                self.reg.A = val;
                self.set_zero_and_negative_flags(val);
                self.reg.P.set_carry_flag(carry);
                self.reg.P.set_overflow_flag(overflow);
            }
            Opcode::CMP => {
                let a = self.reg.A;
                let m = self.load_inst(bus, addr);
                self.comp_inst(a, m);
            }
            Opcode::CPX => {
                let x = self.reg.X;
                let m = self.load_inst(bus, addr);
                self.comp_inst(x, m);
            }
            Opcode::CPY => {
                let y = self.reg.Y;
                let m = self.load_inst(bus, addr);
                self.comp_inst(y, m);
            }
            Opcode::INC => {
                let val = self.load_inst(bus, addr) + 1;
                self.write_inst(bus, addr, val);
                self.set_zero_and_negative_flags(val);
            }
            Opcode::INX => {
                let val = self.reg.X.wrapping_add(1);
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::INY => {
                let val = self.reg.Y.wrapping_add(1);
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::DEC => {
                let val = self.load_inst(bus, addr) - 1;
                self.write_inst(bus, addr, val);
                self.set_zero_and_negative_flags(val);
            }
            Opcode::DEX => {
                let val = self.reg.X.wrapping_sub(1);
                self.reg.X = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::DEY => {
                let val = self.reg.Y.wrapping_sub(1);
                self.reg.Y = val;
                self.set_zero_and_negative_flags(val);
            }
            Opcode::ASL => {
                let val = self.load_inst(bus, addr);
                self.reg.P.set_carry_flag(val & 0x80 != 0);
                self.reg.P.set_negative_flag(val & 0x40 != 0);
                self.reg.P.set_zero_flag(val == 0);
                self.write_inst(bus, addr, (val & 0x7f) << 1);
            }
            Opcode::LSR => {
                let val = self.load_inst(bus, addr);
                self.reg.P.set_carry_flag(val & 0x01 != 0);
                self.reg.P.set_negative_flag(false);
                self.reg.P.set_zero_flag(val == 0);
                self.write_inst(bus, addr, val >> 1);
            }
            Opcode::ROL => {
                let val = self.load_inst(bus, addr);
                self.reg.P.set_carry_flag(val & 0x80 != 0);
                self.reg.P.set_negative_flag(val & 0x40 != 0);
                self.reg.P.set_zero_flag(val == 0);
                self.write_inst(bus, addr, ((val & 0x7f) << 1) | ((val & 0x80) >> 7));
            }
            Opcode::ROR => {
                let val = self.load_inst(bus, addr);
                self.reg.P.set_carry_flag(val & 0x01 != 0);
                self.reg.P.set_negative_flag(val & 0x01 != 0);
                self.reg.P.set_zero_flag(val == 0);
                self.write_inst(bus, addr, (val >> 1) | ((val & 0x01) << 7));
            }
            Opcode::JMP => {
                self.jump_inst(addr);
            }
            Opcode::JSR => {
                let pc = self.reg.PC - 1;
                self.push_stack_w(bus, pc);
                self.jump_inst(addr);
            }
            Opcode::RTS => {
                let addr = self.pop_stack_w(bus);
                self.jump(addr + 1);
            }
            Opcode::BCC => {
                if !self.reg.P.carry_flag() {
                    self.jump_inst(addr);
                }
            }
            Opcode::BCS => {
                if self.reg.P.carry_flag() {
                    self.jump_inst(addr);
                }
            }
            Opcode::BEQ => {
                if self.reg.P.zero_flag() {
                    self.jump_inst(addr);
                }
            }
            Opcode::BMI => {
                if self.reg.P.negative_flag() {
                    self.jump_inst(addr);
                }
            }
            Opcode::BNE => {
                if !self.reg.P.zero_flag() {
                    self.jump_inst(addr);
                }
            }
            Opcode::BPL => {
                if !self.reg.P.negative_flag() {
                    self.jump_inst(addr);
                }
            }
            Opcode::BVC => {
                if !self.reg.P.overflow_flag() {
                    self.jump_inst(addr);
                }
            }
            Opcode::BVS => {
                if self.reg.P.overflow_flag() {
                    self.jump_inst(addr);
                }
            }
            Opcode::CLC => {
                self.reg.P.set_carry_flag(false);
            }
            Opcode::CLD => {
                self.reg.P.set_decimal_mode(false);
            }
            Opcode::CLI => {
                self.reg.P.set_interrupt_disable_flag(false);
            }
            Opcode::CLV => {
                self.reg.P.set_overflow_flag(false);
            }
            Opcode::SEC => {
                self.reg.P.set_carry_flag(true);
            }
            Opcode::SED => {
                self.reg.P.set_decimal_mode(true);
            }
            Opcode::SEI => {
                self.reg.P.set_interrupt_disable_flag(true);
            }
            Opcode::BRK => unimplemented!(),
            Opcode::NOP => {
                // no operation
            }
            Opcode::RTI => unimplemented!(),
        }
    }
}
