pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL,
    ADC, 
    SUB, 
    SBC,
    AND, 
    OR, 
    XOR, 
    CP, 
    INC(IncDecTarget),
    DEC, 
    CCF, 
    SCF, 
    RRA, 
    RLA,
    RRCA, 
    RRLA, 
    CPL, 
    BIT,
    RST,
    SET,
    SRL,
    RR,
    RL,
    RRC,
    RLC(PrefixTarget),
    SRA,
    SLA,
    SWAP,
    JP(JumpTest),
    LD(LoadType),
    CALL(JumpTest),
    RET(JumpTest),
    PUSH(StackTarget),
    POP(StackTarget),
    NOP,
    HALT,
}
impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }
    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => /*TODO: add mapping for the rest of the instructions */ None
        }
    }
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => None,
            0x01 => None,
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x03 => None,
            0x04 => None,
            0x05 => None,
            0x06 => None,
            0x07 => None,
            0x08 => None,
            0x09 => None,
            0x0A => None,
            0x0B => None,
            0x0C => None,
            0x0D => None,
            0x0E => None,
            0x0F => None,
            0x10 => None,
            0x11 => None,
            0x12 => None,
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x14 => None,
            0x15 => None,
            0x16 => None,

            _ => /* TODO: add mappinf for the rest of instructions */ None 
        }
    }
}
enum PrefixTarget {
    B,
}
enum IncDecTarget {
    BC, DE,
}
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}
pub enum LoadByteSource {
    A, B, C, D, E, H , L, D8, HLI
}
pub enum LoadWordTarget {

}
pub enum LoadWordSource {

}
pub enum StackTarget {
    BC,
}
pub enum LoadType {
    // the docs were very confusing here.
    // byte: load a byte from the source to the target
    // word: load a word from the source to the target (need to change the enums here)
    // AFromIndirect, load INTO A from an indirect source 
    // IndirectFromA, load INTO the indirect address with the value from a
    // AFromByte, load into the A from the LAST memory address
    // ByteAddressFromA, load into LAST address from A
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
    /* load */ AFromIndirect(LoadByteSource),
    /* load */ IndirectFromA(),
    AFromByteAddress(),
    ByteAddressFromA()
}


