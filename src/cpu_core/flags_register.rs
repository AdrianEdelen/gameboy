pub const ZERO_FLAG_BYTE_POSITION: u8 = 7;
pub const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
pub const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
pub const CARRY_FLAG_BYTE_POSITION: u8 = 4;

pub enum Flags {
    ZERO, SUBTRACT, HALF_CARRY, CARRY
}
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
}
/*
Flag register {f}
  ┌-> Carry
 ┌-+> Subtraction
 | |
1111 0000
| |
└-+> Zero
  └-> Half Carry
*/
impl FlagsRegister {
    pub fn new() -> Self {
        FlagsRegister { 
            zero: false, 
            subtract: false, 
            half_carry: false, 
            carry: false 
        }
    }

    pub fn get_zero(&self) -> bool {
        self.zero
    }

    pub fn get_subtract(&self) -> bool {
        self.subtract
    }

    pub fn get_half_carry(&self) -> bool {
        self.half_carry
    }

    pub fn get_carry(&self) -> bool {
        self.carry
    }

    pub fn set_zero(&mut self, value: bool) {
        self.zero = value
    }

    pub fn set_subtract(&mut self, value: bool) {
        self.subtract = value
    }

    pub fn set_half_carry(&mut self, value: bool) {
        self.half_carry = value
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry = value
    }

}

impl std::convert::From<FlagsRegister> for u8 {
    fn from (flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION | 
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION 
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
        
    }
}