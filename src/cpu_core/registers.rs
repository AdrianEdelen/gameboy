use crate::cpu_core::FlagsRegister::{FlagsRegister, Flags};

pub enum RegistersU8 {
    A,B,C,D,E,F,H,L,
}

pub enum RegistersU16 {
    BC,HL,
}

pub struct RegisterBank {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8
}

impl RegisterBank {
    
    pub fn new() -> Self {
        RegisterBank {a:0,
            b:0,
            c:0,
            d:0,
            e:0,
            f:FlagsRegister::new(),
            h:0,
            l:0
        }
    }
  
    pub fn get_register_u8(&self, register: RegistersU8) -> u8 {
        match register {
            RegistersU8::A => self.a,
            RegistersU8::B => self.b,
            RegistersU8::C => self.c,
            RegistersU8::D => self.d,
            RegistersU8::E => self.e,
            RegistersU8::F => (&self.f).into(),
            RegistersU8::H => self.h,
            RegistersU8::L => self.l,
        }
    }

    pub fn set_register_u8(&mut self, register: RegistersU8, value: u8) {
        match register {
            RegistersU8::A => self.a = value,
            RegistersU8::B => self.b = value,
            RegistersU8::C => self.c = value,
            RegistersU8::D => self.d = value,
            RegistersU8::E => self.e = value,
            RegistersU8::F => self.f = value.into(),
            RegistersU8::H => self.h = value,
            RegistersU8::L => self.l = value
        }
    }

    pub fn get_register_u16(&self, register: RegistersU16) -> u16 {
        match register {
            RegistersU16::BC => (self.b as u16) << 8 | self.c as u16,
            RegistersU16::HL => (self.h as u16) << 8 | self.l as u16, 
        }
    }

    pub fn set_register_u16(&mut self, register: RegistersU16, value: u16) {
        match register {
            RegistersU16::BC => {
                self.b = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            RegistersU16::HL => {
                self.h = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
        }
    }

    pub fn get_flag(&self, flag_type: Flags) -> bool {
        match flag_type {
            Flags::CARRY => self.f.get_carry(),
            Flags::HALF_CARRY => self.f.get_half_carry(),
            Flags::SUBTRACT => self.f.get_subtract(),
            Flags::ZERO => self.f.get_zero()
        }
    }

    pub fn set_flag(&mut self, flag_type: Flags, value: bool) {
        match flag_type {
            Flags::CARRY => self.f.set_carry(value),
            Flags::HALF_CARRY => self.f.set_half_carry(value),
            Flags::SUBTRACT => self.f.set_subtract(value),
            Flags::ZERO => self.f.set_zero(value),
        }
    }

}
pub const ZERO_FLAG_BYTE_POSITION: u8 = 7;
pub const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
pub const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
pub const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::Into<u8> for &FlagsRegister {
    fn into(self) -> u8 {
        (if self.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if self.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION | 
        (if self.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if self.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION 
    }
}
