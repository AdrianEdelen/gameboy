use crate::cpu_core::instruction::*;
use crate::cpu_core::registers::RegisterBank;
use crate::{cpu_core::memory::MemoryBus, cpu_core::registers::RegistersU16, cpu_core::registers::RegistersU8, cpu_core::FlagsRegister::Flags};

pub struct CPU {
    registers: RegisterBank,
    pc: u16, // program counter
    sp: u16, // stack pointer
    bus: MemoryBus,
    is_halted: bool,
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: RegisterBank::new(),
            pc: 0,
            sp: 0,
            bus: MemoryBus::new(),
            is_halted: false,
        }
    }
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB; // 0xCB is the prefix byte
        if prefixed {
            // if we get a prefix byte we should read the next byte
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unknown instruction found for: {}", description);
        };

        self.pc = next_pc;
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }
    fn read_next_word(&self) -> u16 {
        todo!()
    }
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }
    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }
    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            panic!("Handle Halt appropriately")
        }
        match instruction {
            Instruction::ADD(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.get_register_u8(RegistersU8::A),
                    ArithmeticTarget::B => self.registers.get_register_u8(RegistersU8::B),
                    ArithmeticTarget::C => self.registers.get_register_u8(RegistersU8::C),
                    ArithmeticTarget::D => self.registers.get_register_u8(RegistersU8::D),
                    ArithmeticTarget::E => self.registers.get_register_u8(RegistersU8::E),
                    ArithmeticTarget::H => self.registers.get_register_u8(RegistersU8::H),
                    ArithmeticTarget::L => self.registers.get_register_u8(RegistersU8::L),
                };

                let new_value = self.add(value);
                self.registers.set_register_u8(RegistersU8::A, new_value);
                self.pc.wrapping_add(1)
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.get_flag(Flags::ZERO),
                    JumpTest::NotCarry => !self.registers.get_flag(Flags::CARRY),
                    JumpTest::Zero => self.registers.get_flag(Flags::ZERO),
                    JumpTest::Carry => self.registers.get_flag(Flags::CARRY),
                    JumpTest::Always => true,
                };
                self.jump(jump_condition)
            }
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.get_register_u8(RegistersU8::A),
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_register_u16(RegistersU16::HL)),
                        _ => panic!("TODO: implement other sources")
                    };
                    match target {
                        LoadByteTarget::A => self.registers.set_register_u8(RegistersU8::A, source_value),
                        LoadByteTarget::HLI => {
                            self.bus.write_byte(self.registers.get_register_u16(RegistersU16::HL), source_value)
                        }
                        _ => panic!("TODO: implement other targets")
                    };
                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
                _ => panic!("TODO: implement other load types")
            },
            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::Zero =>     self.registers.get_flag(Flags::ZERO),
                    JumpTest::NotZero =>  self.registers.get_flag(Flags::ZERO),
                    JumpTest::Carry =>    self.registers.get_flag(Flags::CARRY),
                    JumpTest::NotCarry => self.registers.get_flag(Flags::CARRY),
                    JumpTest::Always =>   true,
                };
                self.call(jump_condition)
            }
            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::Zero =>     self.registers.get_flag(Flags::ZERO),
                    JumpTest::NotZero =>  self.registers.get_flag(Flags::ZERO),
                    JumpTest::Carry =>    self.registers.get_flag(Flags::CARRY),
                    JumpTest::NotCarry => self.registers.get_flag(Flags::CARRY),
                    JumpTest::Always => true,
                };
                self.return_(jump_condition)
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_register_u16(RegistersU16::BC),
                    _ => todo!("More Targets")
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_register_u16(RegistersU16::BC, result),
                    _ => todo!("Support more instructions")
                    
                };
                self.pc.wrapping_add(1)
            }
            Instruction::NOP => {
                self.pc.wrapping_add(1)
            }
            Instruction::HALT => {
                self.is_halted = true;
                panic!("reach halt instruction")
            }
            _ => {
                panic!("unsupported instruction encountered");
                /* TODO: support more instructions */
                // self.pc
            }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.get_register_u8(RegistersU8::A).overflowing_add(value);
        self.registers.set_flag(Flags::ZERO, new_value == 0);
        self.registers.set_flag(Flags::SUBTRACT, false);        
        self.registers.set_flag(Flags::CARRY, did_overflow);        
        /* half carry: set to true if there is an overflow from the lowe nibble to the upper nibble
           lower nibble            lower nibble
                   ┌--┐                    ┌--┐
               1000 1111  +   1   ==   1001 0000
               └--┘                    └--┘
           upper nibble            upper nibble
        */
        self.registers.set_flag(Flags::HALF_CARRY, (self.registers.get_register_u8(RegistersU8::A) & 0xF) + (value & 0xF) > 0xF);
        new_value
    }
    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most sig
            // and pc + 1 as least sig
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // if we don't jump we need to still move the pc fwd by 3 to account for the jp instr
            self.pc.wrapping_add(3)
        }
    }
    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }
    fn return_ (&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
    fn addhl(&mut self) {} // add to hl register
    fn adc(&mut self) {} // add with carry
    fn sub(&mut self) {} // subtract value with A register
    fn sbc(&mut self) {} // subtract with carry
    fn and(&mut self) {} // bitwise and value and A register
    fn or(&mut self) {} // bitwise or with value and A register
    fn xor(&mut self) {} // bitwise exclusive or with value and A register
    fn cp(&mut self) {} // compare (subtract but the result is not stored in the A register)
    fn inc(&mut self) {} // increment the value in a given register by 1
    fn dec(&mut self) {} // decrement the value in a given register by 1
    fn ccf(&mut self) {} // complement carry flag:  toggle the value of the carry flag
    fn scf(&mut self) {} // set the carry flag to true
    fn rra(&mut self) {} // RRA (rotate right A register) - bit rotate A register right through the carry flag
    fn rla(&mut self) {} // RLA (rotate left A register) - bit rotate A register left through the carry flag
    fn rrca(&mut self) {} // RRCA (rotate right A register) - bit rotate A register right (not through the carry flag)
    fn rrla(&mut self) {} // RRLA (rotate left A register) - bit rotate A register left (not through the carry flag)
    fn cpl(&mut self) {} // CPL (complement) - toggle every bit of the A register
    fn bit(&mut self) {} // BIT (bit test) - test to see if a specific bit of a specific register is set
    fn rst(&mut self) {} // RESET (bit reset) - set a specific bit of a specific register to 0
    fn set(&mut self) {} // SET (bit set) - set a specific bit of a specific register to 1
    fn srl(&mut self) {} // SRL (shift right logical) - bit shift a specific register right by 1
    fn rr(&mut self) {} // RR (rotate right) - bit rotate a specific register right by 1 through the carry flag
    fn rl(&mut self) {} // RL (rotate left) - bit rotate a specific register left by 1 through the carry flag
    fn rrc(&mut self) {} // RRC (rorate right) - bit rotate a specific register right by 1 (not through the carry flag)
    fn rlc(&mut self) {} // RLC (rorate left) - bit rotate a specific register left by 1 (not through the carry flag)
    fn sra(&mut self) {} // SRA (shift right arithmetic) - arithmetic shift a specific register right by 1
    fn sla(&mut self) {} // SLA (shift left arithmetic) - arithmetic shift a specific register left by 1
    fn swap(&mut self) {} // SWAP (swap nibbles) - switch upper and lower nibble of a specific register
}
