#[derive(Debug)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticSource),
    ADC(), 
    SUB(), 
    SBC(),
    AND(), 
    OR(), 
    XOR(), 
    CP(), 
    INC(IncDecTarget),
    DEC(IncDecTarget), 
    CCF(), 
    SCF(), 
    RRA(), 
    RLA(),
    RRCA, 
    RRLA(), 
    CPL(), 
    BIT(u8, PrefixTarget),
    RST(),
    SET(u8, PrefixTarget),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),
    JP(JumpTest),
    LD(LoadType),
    CALL(JumpTest),
    RET(JumpTest),
    PUSH(StackTarget),
    POP(StackTarget),
    NOP,
    HALT,
    RES(u8, PrefixTarget),
    RLCA,
    STOP(u8)
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
            0x00..=0x0f => Self::handle_byte_00_to_0f_prefixed(byte), 
            0x10..=0x1f => Self::handle_byte_10_to_1f_prefixed(byte),
            0x21..=0x30 => Self::handle_byte_20_to_2f_prefixed(byte),
            0x31..=0x40 => Self::handle_byte_30_to_3f_prefixed(byte),
            0x41..=0x50 => Self::handle_byte_40_to_4f_prefixed(byte),
            0x51..=0x60 => Self::handle_byte_50_to_5f_prefixed(byte),
            0x61..=0x70 => Self::handle_byte_60_to_6f_prefixed(byte),
            0x71..=0x80 => Self::handle_byte_70_to_7f_prefixed(byte),
            0x81..=0x90 => Self::handle_byte_80_to_8f_prefixed(byte),
            0x91..=0xa0 => Self::handle_byte_90_to_9f_prefixed(byte),
            0xa1..=0xb0 => Self::handle_byte_a0_to_af_prefixed(byte),
            0xb1..=0xc0 => Self::handle_byte_b0_to_bf_prefixed(byte),
            0xc1..=0xd0 => Self::handle_byte_c0_to_cf_prefixed(byte),
            0xd1..=0xe0 => Self::handle_byte_d0_to_df_prefixed(byte),
            0xe1..=0xf0 => Self::handle_byte_e0_to_ef_prefixed(byte),
            0xf1..=0xff => Self::handle_byte_f0_to_ff_prefixed(byte),
            _ => panic!("no Instruction map for: prefixed 0x{:x}", byte),
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00..=0x10 => Self::handle_byte_00_to_0f(byte), 
            0x11..=0x20 => Self::handle_byte_10_to_1f(byte),
            0x21..=0x30 => Self::handle_byte_20_to_2f(byte),
            0x31..=0x40 => Self::handle_byte_30_to_3f(byte),
            0x41..=0x50 => Self::handle_byte_40_to_4f(byte),
            0x51..=0x60 => Self::handle_byte_50_to_5f(byte),
            0x61..=0x70 => Self::handle_byte_60_to_6f(byte),
            0x71..=0x80 => Self::handle_byte_70_to_7f(byte),
            0x81..=0x90 => Self::handle_byte_80_to_8f(byte),
            0x91..=0xa0 => Self::handle_byte_90_to_9f(byte),
            0xa1..=0xb0 => Self::handle_byte_a0_to_af(byte),
            0xb1..=0xc0 => Self::handle_byte_b0_to_bf(byte),
            0xc1..=0xd0 => Self::handle_byte_c0_to_cf(byte),
            0xd1..=0xe0 => Self::handle_byte_d0_to_df(byte),
            0xe1..=0xf0 => Self::handle_byte_e0_to_ef(byte),
            0xf1..=0xff => Self::handle_byte_f0_to_ff(byte),
            _ => panic!("no Instruction map for: 0x{:x}", byte) 
        }
    }

    fn handle_byte_00_to_0f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            0x01 => Some(Instruction::RLC(PrefixTarget::C)),
            0x02 => Some(Instruction::RLC(PrefixTarget::D)),
            0x03 => Some(Instruction::RLC(PrefixTarget::E)),
            0x04 => Some(Instruction::RLC(PrefixTarget::H)),
            0x05 => Some(Instruction::RLC(PrefixTarget::L)),
            0x06 => Some(Instruction::RLC(PrefixTarget::HL)),
            0x07 => Some(Instruction::RLC(PrefixTarget::A)),
            0x08 => Some(Instruction::RRC(PrefixTarget::B)),
            0x09 => Some(Instruction::RRC(PrefixTarget::C)),
            0x0A => Some(Instruction::RRC(PrefixTarget::D)),
            0x0B => Some(Instruction::RRC(PrefixTarget::E)),
            0x0C => Some(Instruction::RRC(PrefixTarget::H)),
            0x0D => Some(Instruction::RRC(PrefixTarget::L)),
            0x0E => Some(Instruction::RRC(PrefixTarget::HL)),
            0x0F => Some(Instruction::RRC(PrefixTarget::A)),
        }
    }
    
    fn handle_byte_10_to_1f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x10 => Some(Instruction::RL(PrefixTarget::B)),
            0x11 => Some(Instruction::RL(PrefixTarget::C)),
            0x12 => Some(Instruction::RL(PrefixTarget::D)),
            0x13 => Some(Instruction::RL(PrefixTarget::E)),
            0x14 => Some(Instruction::RL(PrefixTarget::H)),
            0x15 => Some(Instruction::RL(PrefixTarget::L)),
            0x16 => Some(Instruction::RL(PrefixTarget::HL)),
            0x17 => Some(Instruction::RL(PrefixTarget::A)),
            0x18 => Some(Instruction::RR(PrefixTarget::B)),
            0x19 => Some(Instruction::RR(PrefixTarget::C)),
            0x1A => Some(Instruction::RR(PrefixTarget::D)),
            0x1B => Some(Instruction::RR(PrefixTarget::E)),
            0x1C => Some(Instruction::RR(PrefixTarget::H)),
            0x1D => Some(Instruction::RR(PrefixTarget::L)),
            0x1E => Some(Instruction::RR(PrefixTarget::HL)),
            0x1F => Some(Instruction::RR(PrefixTarget::A)),
        }
    }
    
    fn handle_byte_20_to_2f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x20 => Some(Instruction::SLA(PrefixTarget::B)),
            0x21 => Some(Instruction::SLA(PrefixTarget::C)),
            0x22 => Some(Instruction::SLA(PrefixTarget::D)),
            0x23 => Some(Instruction::SLA(PrefixTarget::E)),
            0x24 => Some(Instruction::SLA(PrefixTarget::H)),
            0x25 => Some(Instruction::SLA(PrefixTarget::L)),
            0x26 => Some(Instruction::SLA(PrefixTarget::HL)),
            0x27 => Some(Instruction::SLA(PrefixTarget::A)),
            0x28 => Some(Instruction::SRA(PrefixTarget::B)),
            0x29 => Some(Instruction::SRA(PrefixTarget::C)),
            0x2A => Some(Instruction::SRA(PrefixTarget::D)),
            0x2B => Some(Instruction::SRA(PrefixTarget::E)),
            0x2C => Some(Instruction::SRA(PrefixTarget::H)),
            0x2D => Some(Instruction::SRA(PrefixTarget::L)),
            0x2E => Some(Instruction::SRA(PrefixTarget::HL)),
            0x2F => Some(Instruction::SRA(PrefixTarget::A)),
        }
    }
    
    fn handle_byte_30_to_3f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x30 => Some(Instruction::SWAP(PrefixTarget::B)),
            0x31 => Some(Instruction::SWAP(PrefixTarget::C)),
            0x32 => Some(Instruction::SWAP(PrefixTarget::D)),
            0x33 => Some(Instruction::SWAP(PrefixTarget::E)),
            0x34 => Some(Instruction::SWAP(PrefixTarget::H)),
            0x35 => Some(Instruction::SWAP(PrefixTarget::L)),
            0x36 => Some(Instruction::SWAP(PrefixTarget::HL)),
            0x37 => Some(Instruction::SWAP(PrefixTarget::A)),
            0x38 => Some(Instruction::SRL(PrefixTarget::B)),
            0x39 => Some(Instruction::SRL(PrefixTarget::C)),
            0x3A => Some(Instruction::SRL(PrefixTarget::D)),
            0x3B => Some(Instruction::SRL(PrefixTarget::E)),
            0x3C => Some(Instruction::SRL(PrefixTarget::H)),
            0x3D => Some(Instruction::SRL(PrefixTarget::L)),
            0x3E => Some(Instruction::SRL(PrefixTarget::HL)),
            0x3F => Some(Instruction::SRL(PrefixTarget::A)),
        }
    }
    
    fn handle_byte_40_to_4f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x40 => Some(Instruction::BIT(0, PrefixTarget::B)),
            0x41 => Some(Instruction::BIT(0, PrefixTarget::C)),
            0x42 => Some(Instruction::BIT(0, PrefixTarget::D)),
            0x43 => Some(Instruction::BIT(0, PrefixTarget::E)),
            0x44 => Some(Instruction::BIT(0, PrefixTarget::H)),
            0x45 => Some(Instruction::BIT(0, PrefixTarget::L)),
            0x46 => Some(Instruction::BIT(0, PrefixTarget::HL)),
            0x47 => Some(Instruction::BIT(0, PrefixTarget::A)),
            0x48 => Some(Instruction::BIT(1, PrefixTarget::B)),
            0x49 => Some(Instruction::BIT(1, PrefixTarget::C)),
            0x4A => Some(Instruction::BIT(1, PrefixTarget::D)),
            0x4B => Some(Instruction::BIT(1, PrefixTarget::E)),
            0x4C => Some(Instruction::BIT(1, PrefixTarget::H)),
            0x4D => Some(Instruction::BIT(1, PrefixTarget::L)),
            0x4E => Some(Instruction::BIT(1, PrefixTarget::HL)),
            0x4F => Some(Instruction::BIT(1, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_50_to_5f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x50 => Some(Instruction::BIT(2, PrefixTarget::B)),
            0x51 => Some(Instruction::BIT(2, PrefixTarget::C)),
            0x52 => Some(Instruction::BIT(2, PrefixTarget::D)),
            0x53 => Some(Instruction::BIT(2, PrefixTarget::E)),
            0x54 => Some(Instruction::BIT(2, PrefixTarget::H)),
            0x55 => Some(Instruction::BIT(2, PrefixTarget::L)),
            0x56 => Some(Instruction::BIT(2, PrefixTarget::HL)),
            0x57 => Some(Instruction::BIT(2, PrefixTarget::A)),
            0x58 => Some(Instruction::BIT(3, PrefixTarget::B)),
            0x59 => Some(Instruction::BIT(3, PrefixTarget::C)),
            0x5A => Some(Instruction::BIT(3, PrefixTarget::D)),
            0x5B => Some(Instruction::BIT(3, PrefixTarget::E)),
            0x5C => Some(Instruction::BIT(3, PrefixTarget::H)),
            0x5D => Some(Instruction::BIT(3, PrefixTarget::L)),
            0x5E => Some(Instruction::BIT(3, PrefixTarget::HL)),
            0x5F => Some(Instruction::BIT(3, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_60_to_6f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x60 => Some(Instruction::BIT(4, PrefixTarget::B)),
            0x61 => Some(Instruction::BIT(4, PrefixTarget::C)),
            0x62 => Some(Instruction::BIT(4, PrefixTarget::D)),
            0x63 => Some(Instruction::BIT(4, PrefixTarget::E)),
            0x64 => Some(Instruction::BIT(4, PrefixTarget::H)),
            0x65 => Some(Instruction::BIT(4, PrefixTarget::L)),
            0x66 => Some(Instruction::BIT(4, PrefixTarget::HL)),
            0x67 => Some(Instruction::BIT(4, PrefixTarget::A)),
            0x68 => Some(Instruction::BIT(5, PrefixTarget::B)),
            0x69 => Some(Instruction::BIT(5, PrefixTarget::C)),
            0x6A => Some(Instruction::BIT(5, PrefixTarget::D)),
            0x6B => Some(Instruction::BIT(5, PrefixTarget::E)),
            0x6C => Some(Instruction::BIT(5, PrefixTarget::H)),
            0x6D => Some(Instruction::BIT(5, PrefixTarget::L)),
            0x6E => Some(Instruction::BIT(5, PrefixTarget::HL)),
            0x6F => Some(Instruction::BIT(5, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_70_to_7f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x70 => Some(Instruction::BIT(6, PrefixTarget::B)),
            0x71 => Some(Instruction::BIT(6, PrefixTarget::C)),
            0x72 => Some(Instruction::BIT(6, PrefixTarget::D)),
            0x73 => Some(Instruction::BIT(6, PrefixTarget::E)),
            0x74 => Some(Instruction::BIT(6, PrefixTarget::H)),
            0x75 => Some(Instruction::BIT(6, PrefixTarget::L)),
            0x76 => Some(Instruction::BIT(6, PrefixTarget::HL)),
            0x77 => Some(Instruction::BIT(6, PrefixTarget::A)),
            0x78 => Some(Instruction::BIT(7, PrefixTarget::B)),
            0x79 => Some(Instruction::BIT(7, PrefixTarget::C)),
            0x7A => Some(Instruction::BIT(7, PrefixTarget::D)),
            0x7B => Some(Instruction::BIT(7, PrefixTarget::E)),
            0x7C => Some(Instruction::BIT(7, PrefixTarget::H)),
            0x7D => Some(Instruction::BIT(7, PrefixTarget::L)),
            0x7E => Some(Instruction::BIT(7, PrefixTarget::HL)),
            0x7F => Some(Instruction::BIT(7, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_80_to_8f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x80 => Some(Instruction::RES(0, PrefixTarget::B)),
            0x81 => Some(Instruction::RES(0, PrefixTarget::C)),
            0x82 => Some(Instruction::RES(0, PrefixTarget::D)),
            0x83 => Some(Instruction::RES(0, PrefixTarget::E)),
            0x84 => Some(Instruction::RES(0, PrefixTarget::H)),
            0x85 => Some(Instruction::RES(0, PrefixTarget::L)),
            0x86 => Some(Instruction::RES(0, PrefixTarget::HL)),
            0x87 => Some(Instruction::RES(0, PrefixTarget::A)),
            0x88 => Some(Instruction::RES(1, PrefixTarget::B)),
            0x89 => Some(Instruction::RES(1, PrefixTarget::C)),
            0x8A => Some(Instruction::RES(1, PrefixTarget::D)),
            0x8B => Some(Instruction::RES(1, PrefixTarget::E)),
            0x8C => Some(Instruction::RES(1, PrefixTarget::H)),
            0x8D => Some(Instruction::RES(1, PrefixTarget::L)),
            0x8E => Some(Instruction::RES(1, PrefixTarget::HL)),
            0x8F => Some(Instruction::RES(1, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_90_to_9f_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x90 => Some(Instruction::RES(2, PrefixTarget::B)),
            0x91 => Some(Instruction::RES(2, PrefixTarget::C)),
            0x92 => Some(Instruction::RES(2, PrefixTarget::D)),
            0x93 => Some(Instruction::RES(2, PrefixTarget::E)),
            0x94 => Some(Instruction::RES(2, PrefixTarget::H)),
            0x95 => Some(Instruction::RES(2, PrefixTarget::L)),
            0x96 => Some(Instruction::RES(2, PrefixTarget::HL)),
            0x97 => Some(Instruction::RES(2, PrefixTarget::A)),
            0x98 => Some(Instruction::RES(3, PrefixTarget::B)),
            0x99 => Some(Instruction::RES(3, PrefixTarget::C)),
            0x9A => Some(Instruction::RES(3, PrefixTarget::D)),
            0x9B => Some(Instruction::RES(3, PrefixTarget::E)),
            0x9C => Some(Instruction::RES(3, PrefixTarget::H)),
            0x9D => Some(Instruction::RES(3, PrefixTarget::L)),
            0x9E => Some(Instruction::RES(3, PrefixTarget::HL)),
            0x9F => Some(Instruction::RES(3, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_a0_to_af_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0xA0 => Some(Instruction::RES(4, PrefixTarget::B)),
            0xA1 => Some(Instruction::RES(4, PrefixTarget::C)),
            0xA2 => Some(Instruction::RES(4, PrefixTarget::D)),
            0xA3 => Some(Instruction::RES(4, PrefixTarget::E)),
            0xA4 => Some(Instruction::RES(4, PrefixTarget::H)),
            0xA5 => Some(Instruction::RES(4, PrefixTarget::L)),
            0xA6 => Some(Instruction::RES(4, PrefixTarget::HL)),
            0xA7 => Some(Instruction::RES(4, PrefixTarget::A)),
            0xA8 => Some(Instruction::RES(5, PrefixTarget::B)),
            0xA9 => Some(Instruction::RES(5, PrefixTarget::C)),
            0xAA => Some(Instruction::RES(5, PrefixTarget::D)),
            0xAB => Some(Instruction::RES(5, PrefixTarget::E)),
            0xAC => Some(Instruction::RES(5, PrefixTarget::H)),
            0xAD => Some(Instruction::RES(5, PrefixTarget::L)),
            0xAE => Some(Instruction::RES(5, PrefixTarget::HL)),
            0xAF => Some(Instruction::RES(5, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_b0_to_bf_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0xB0 => Some(Instruction::RES(6, PrefixTarget::B)),
            0xB1 => Some(Instruction::RES(6, PrefixTarget::C)),
            0xB2 => Some(Instruction::RES(6, PrefixTarget::D)),
            0xB3 => Some(Instruction::RES(6, PrefixTarget::E)),
            0xB4 => Some(Instruction::RES(6, PrefixTarget::H)),
            0xB5 => Some(Instruction::RES(6, PrefixTarget::L)),
            0xB6 => Some(Instruction::RES(6, PrefixTarget::HL)),
            0xB7 => Some(Instruction::RES(6, PrefixTarget::A)),
            0xB8 => Some(Instruction::RES(7, PrefixTarget::B)),
            0xB9 => Some(Instruction::RES(7, PrefixTarget::C)),
            0xBA => Some(Instruction::RES(7, PrefixTarget::D)),
            0xBB => Some(Instruction::RES(7, PrefixTarget::E)),
            0xBC => Some(Instruction::RES(7, PrefixTarget::H)),
            0xBD => Some(Instruction::RES(7, PrefixTarget::L)),
            0xBE => Some(Instruction::RES(7, PrefixTarget::HL)),
            0xBF => Some(Instruction::RES(7, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_c0_to_cf_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0xC0 => Some(Instruction::SET(0, PrefixTarget::B)),
            0xC1 => Some(Instruction::SET(0, PrefixTarget::C)),
            0xC2 => Some(Instruction::SET(0, PrefixTarget::D)),
            0xC3 => Some(Instruction::SET(0, PrefixTarget::E)),
            0xC4 => Some(Instruction::SET(0, PrefixTarget::H)),
            0xC5 => Some(Instruction::SET(0, PrefixTarget::L)),
            0xC6 => Some(Instruction::SET(0, PrefixTarget::HL)),
            0xC7 => Some(Instruction::SET(0, PrefixTarget::A)),
            0xC8 => Some(Instruction::SET(1, PrefixTarget::B)),
            0xC9 => Some(Instruction::SET(1, PrefixTarget::C)),
            0xCA => Some(Instruction::SET(1, PrefixTarget::D)),
            0xCB => Some(Instruction::SET(1, PrefixTarget::E)),
            0xCC => Some(Instruction::SET(1, PrefixTarget::H)),
            0xCD => Some(Instruction::SET(1, PrefixTarget::L)),
            0xCE => Some(Instruction::SET(1, PrefixTarget::HL)),
            0xCF => Some(Instruction::SET(1, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_d0_to_df_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0xD0 => Some(Instruction::SET(2, PrefixTarget::B)),
            0xD1 => Some(Instruction::SET(2, PrefixTarget::C)),
            0xD2 => Some(Instruction::SET(2, PrefixTarget::D)),
            0xD3 => Some(Instruction::SET(2, PrefixTarget::E)),
            0xD4 => Some(Instruction::SET(2, PrefixTarget::H)),
            0xD5 => Some(Instruction::SET(2, PrefixTarget::L)),
            0xD6 => Some(Instruction::SET(2, PrefixTarget::HL)),
            0xD7 => Some(Instruction::SET(2, PrefixTarget::A)),
            0xD8 => Some(Instruction::SET(3, PrefixTarget::B)),
            0xD9 => Some(Instruction::SET(3, PrefixTarget::C)),
            0xDA => Some(Instruction::SET(3, PrefixTarget::D)),
            0xDB => Some(Instruction::SET(3, PrefixTarget::E)),
            0xDC => Some(Instruction::SET(3, PrefixTarget::H)),
            0xDD => Some(Instruction::SET(3, PrefixTarget::L)),
            0xDE => Some(Instruction::SET(3, PrefixTarget::HL)),
            0xDF => Some(Instruction::SET(3, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_e0_to_ef_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0xE0 => Some(Instruction::SET(4, PrefixTarget::B)),
            0xE1 => Some(Instruction::SET(4, PrefixTarget::C)),
            0xE2 => Some(Instruction::SET(4, PrefixTarget::D)),
            0xE3 => Some(Instruction::SET(4, PrefixTarget::E)),
            0xE4 => Some(Instruction::SET(4, PrefixTarget::H)),
            0xE5 => Some(Instruction::SET(4, PrefixTarget::L)),
            0xE6 => Some(Instruction::SET(4, PrefixTarget::HL)),
            0xE7 => Some(Instruction::SET(4, PrefixTarget::A)),
            0xE8 => Some(Instruction::SET(5, PrefixTarget::B)),
            0xE9 => Some(Instruction::SET(5, PrefixTarget::C)),
            0xEA => Some(Instruction::SET(5, PrefixTarget::D)),
            0xEB => Some(Instruction::SET(5, PrefixTarget::E)),
            0xEC => Some(Instruction::SET(5, PrefixTarget::H)),
            0xED => Some(Instruction::SET(5, PrefixTarget::L)),
            0xEE => Some(Instruction::SET(5, PrefixTarget::HL)),
            0xEF => Some(Instruction::SET(5, PrefixTarget::A)),
        }
    }
    
    fn handle_byte_f0_to_ff_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0xF0 => Some(Instruction::SET(6, PrefixTarget::B)),
            0xF1 => Some(Instruction::SET(6, PrefixTarget::C)),
            0xF2 => Some(Instruction::SET(6, PrefixTarget::D)),
            0xF3 => Some(Instruction::SET(6, PrefixTarget::E)),
            0xF4 => Some(Instruction::SET(6, PrefixTarget::H)),
            0xF5 => Some(Instruction::SET(6, PrefixTarget::L)),
            0xF6 => Some(Instruction::SET(6, PrefixTarget::HL)),
            0xF7 => Some(Instruction::SET(6, PrefixTarget::A)),
            0xF8 => Some(Instruction::SET(7, PrefixTarget::B)),
            0xF9 => Some(Instruction::SET(7, PrefixTarget::C)),
            0xFA => Some(Instruction::SET(7, PrefixTarget::D)),
            0xFB => Some(Instruction::SET(7, PrefixTarget::E)),
            0xFC => Some(Instruction::SET(7, PrefixTarget::H)),
            0xFD => Some(Instruction::SET(7, PrefixTarget::L)),
            0xFE => Some(Instruction::SET(7, PrefixTarget::HL)),
            0xFF => Some(Instruction::SET(7, PrefixTarget::A)),
        }
    }

    fn handle_byte_00_to_0f(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x01 => Some(Instruction::LD(LoadType::Word(LoadTarget::BC, LoadSource::D16))),
            0x02 => Some(Instruction::LD(LoadType::IndirectFromA(LoadTarget::BC, LoadSource::A))),
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadTarget::B, LoadSource::D8))),
            0x07 => Some(Instruction::RLCA),
            0x08 => Some(Instruction::LD(LoadType::IndirectFromByte(LoadTarget::D16, LoadSource::SP))), // load the sp into the indirect (D16)
            0x09 => Some(Instruction::ADDHL(ArithmeticSource::BC)),
            0x0A => Some(Instruction::LD(LoadType::AFromIndirect(LoadTarget::A, LoadSource::BC))),
            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadTarget::C, LoadSource::D8))),
            0x0F => Some(Instruction::RRCA),
        }
    }
    
    fn handle_byte_10_to_1f(byte: u8) -> Option<Instruction> {
        match byte {
            0x10 => Some(Instruction::STOP(0)),
            0x11 => Some(Instruction::),
            0x12 => Some(Instruction::),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x14 => Some(Instruction::),
            0x15 => Some(Instruction::),
            0x16 => Some(Instruction::),
            0x17 => Some(Instruction::),
            0x18 => Some(Instruction::),
            0x19 => Some(Instruction::),
            0x1A => Some(Instruction::),
            0x1B => Some(Instruction::),
            0x1C => Some(Instruction::),
            0x1D => Some(Instruction::),
            0x1E => Some(Instruction::),
            0x1F => Some(Instruction::),
        }
    }
    
    fn handle_byte_20_to_2f(byte: u8) -> Option<Instruction> {
        match byte {
            0x20 => Some(Instruction::),
            0x21 => Some(Instruction::),
            0x22 => Some(Instruction::),
            0x23 => Some(Instruction::),
            0x24 => Some(Instruction::),
            0x25 => Some(Instruction::),
            0x26 => Some(Instruction::),
            0x27 => Some(Instruction::),
            0x28 => Some(Instruction::),
            0x29 => Some(Instruction::),
            0x2A => Some(Instruction::),
            0x2B => Some(Instruction::),
            0x2C => Some(Instruction::),
            0x2D => Some(Instruction::),
            0x2E => Some(Instruction::),
            0x2F => Some(Instruction::),
        }
    }
    
    fn handle_byte_30_to_3f(byte: u8) -> Option<Instruction> {
        match byte {
            0x30 => Some(Instruction::),
            0x31 => Some(Instruction::),
            0x32 => Some(Instruction::),
            0x33 => Some(Instruction::),
            0x34 => Some(Instruction::),
            0x35 => Some(Instruction::),
            0x36 => Some(Instruction::),
            0x37 => Some(Instruction::),
            0x38 => Some(Instruction::),
            0x39 => Some(Instruction::),
            0x3A => Some(Instruction::),
            0x3B => Some(Instruction::),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),
            0x3D => Some(Instruction::),
            0x3E => Some(Instruction::),
            0x3F => Some(Instruction::),
        }
    }
    
    fn handle_byte_40_to_4f(byte: u8) -> Option<Instruction> {
        match byte {
            0x40 => Some(Instruction::),
            0x41 => Some(Instruction::),
            0x42 => Some(Instruction::),
            0x43 => Some(Instruction::),
            0x44 => Some(Instruction::),
            0x45 => Some(Instruction::),
            0x46 => Some(Instruction::),
            0x47 => Some(Instruction::),
            0x48 => Some(Instruction::),
            0x49 => Some(Instruction::),
            0x4A => Some(Instruction::),
            0x4B => Some(Instruction::),
            0x4C => Some(Instruction::),
            0x4D => Some(Instruction::),
            0x4E => Some(Instruction::),
            0x4F => Some(Instruction::),
        }
    }
    
    fn handle_byte_50_to_5f(byte: u8) -> Option<Instruction> {
        match byte {
            0x50 => Some(Instruction::),
            0x51 => Some(Instruction::),
            0x52 => Some(Instruction::),
            0x53 => Some(Instruction::),
            0x54 => Some(Instruction::),
            0x55 => Some(Instruction::),
            0x56 => Some(Instruction::),
            0x57 => Some(Instruction::),
            0x58 => Some(Instruction::),
            0x59 => Some(Instruction::),
            0x5A => Some(Instruction::),
            0x5B => Some(Instruction::),
            0x5C => Some(Instruction::),
            0x5D => Some(Instruction::),
            0x5E => Some(Instruction::),
            0x5F => Some(Instruction::),
        }
    }
    
    fn handle_byte_60_to_6f(byte: u8) -> Option<Instruction> {
        match byte {
            0x60 => Some(Instruction::),
            0x61 => Some(Instruction::),
            0x62 => Some(Instruction::),
            0x63 => Some(Instruction::),
            0x64 => Some(Instruction::),
            0x65 => Some(Instruction::),
            0x66 => Some(Instruction::),
            0x67 => Some(Instruction::),
            0x68 => Some(Instruction::),
            0x69 => Some(Instruction::),
            0x6A => Some(Instruction::),
            0x6B => Some(Instruction::),
            0x6C => Some(Instruction::),
            0x6D => Some(Instruction::),
            0x6E => Some(Instruction::),
            0x6F => Some(Instruction::),
        }
    }
    
    fn handle_byte_70_to_7f(byte: u8) -> Option<Instruction> {
        match byte {
            0x70 => Some(Instruction::),
            0x71 => Some(Instruction::),
            0x72 => Some(Instruction::),
            0x73 => Some(Instruction::),
            0x74 => Some(Instruction::),
            0x75 => Some(Instruction::),
            0x76 => Some(Instruction::),
            0x77 => Some(Instruction::),
            0x78 => Some(Instruction::),
            0x79 => Some(Instruction::),
            0x7A => Some(Instruction::),
            0x7B => Some(Instruction::),
            0x7C => Some(Instruction::),
            0x7D => Some(Instruction::),
            0x7E => Some(Instruction::),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadTarget::A, LoadSource::A))),
        }
    }
    
    fn handle_byte_80_to_8f(byte: u8) -> Option<Instruction> {
        match byte {
            0x80 => Some(Instruction::),
            0x81 => Some(Instruction::),
            0x82 => Some(Instruction::),
            0x83 => Some(Instruction::),
            0x84 => Some(Instruction::),
            0x85 => Some(Instruction::),
            0x86 => Some(Instruction::),
            0x87 => Some(Instruction::),
            0x88 => Some(Instruction::),
            0x89 => Some(Instruction::),
            0x8A => Some(Instruction::),
            0x8B => Some(Instruction::),
            0x8C => Some(Instruction::),
            0x8D => Some(Instruction::),
            0x8E => Some(Instruction::),
            0x8F => Some(Instruction::),
        }
    }
    
    fn handle_byte_90_to_9f(byte: u8) -> Option<Instruction> {
        match byte {
            0x90 => Some(Instruction::),
            0x91 => Some(Instruction::),
            0x92 => Some(Instruction::),
            0x93 => Some(Instruction::),
            0x94 => Some(Instruction::),
            0x95 => Some(Instruction::),
            0x96 => Some(Instruction::),
            0x97 => Some(Instruction::),
            0x98 => Some(Instruction::),
            0x99 => Some(Instruction::),
            0x9A => Some(Instruction::),
            0x9B => Some(Instruction::),
            0x9C => Some(Instruction::),
            0x9D => Some(Instruction::),
            0x9E => Some(Instruction::),
            0x9F => Some(Instruction::),
        }
    }
    
    fn handle_byte_a0_to_af(byte: u8) -> Option<Instruction> {
        match byte {
            0xA0 => Some(Instruction::),
            0xA1 => Some(Instruction::),
            0xA2 => Some(Instruction::),
            0xA3 => Some(Instruction::),
            0xA4 => Some(Instruction::),
            0xA5 => Some(Instruction::),
            0xA6 => Some(Instruction::),
            0xA7 => Some(Instruction::),
            0xA8 => Some(Instruction::),
            0xA9 => Some(Instruction::),
            0xAA => Some(Instruction::),
            0xAB => Some(Instruction::),
            0xAC => Some(Instruction::),
            0xAD => Some(Instruction::),
            0xAE => Some(Instruction::),
            0xAF => Some(Instruction::),
        }
    }
    
    fn handle_byte_b0_to_bf(byte: u8) -> Option<Instruction> {
        match byte {
            0xB0 => Some(Instruction::),
            0xB1 => Some(Instruction::),
            0xB2 => Some(Instruction::),
            0xB3 => Some(Instruction::),
            0xB4 => Some(Instruction::),
            0xB5 => Some(Instruction::),
            0xB6 => Some(Instruction::),
            0xB7 => Some(Instruction::),
            0xB8 => Some(Instruction::),
            0xB9 => Some(Instruction::),
            0xBA => Some(Instruction::),
            0xBB => Some(Instruction::),
            0xBC => Some(Instruction::),
            0xBD => Some(Instruction::),
            0xBE => Some(Instruction::),
            0xBF => Some(Instruction::),
        }
    }
    
    fn handle_byte_c0_to_cf(byte: u8) -> Option<Instruction> {
        match byte {
            0xC0 => Some(Instruction::),
            0xC1 => Some(Instruction::),
            0xC2 => Some(Instruction::),
            0xC3 => Some(Instruction::),
            0xC4 => Some(Instruction::),
            0xC5 => Some(Instruction::),
            0xC6 => Some(Instruction::),
            0xC7 => Some(Instruction::),
            0xC8 => Some(Instruction::),
            0xC9 => Some(Instruction::),
            0xCA => Some(Instruction::),
            0xCB => Some(Instruction::),
            0xCC => Some(Instruction::),
            0xCD => Some(Instruction::),
            0xCE => Some(Instruction::),
            0xCF => Some(Instruction::),
        }
    }
    
    fn handle_byte_d0_to_df(byte: u8) -> Option<Instruction> {
        match byte {
            0xD0 => Some(Instruction::),
            0xD1 => Some(Instruction::),
            0xD2 => Some(Instruction::),
            0xD3 => Some(Instruction::),
            0xD4 => Some(Instruction::),
            0xD5 => Some(Instruction::),
            0xD6 => Some(Instruction::),
            0xD7 => Some(Instruction::),
            0xD8 => Some(Instruction::),
            0xD9 => Some(Instruction::),
            0xDA => Some(Instruction::),
            0xDB => Some(Instruction::),
            0xDC => Some(Instruction::),
            0xDD => Some(Instruction::),
            0xDE => Some(Instruction::),
            0xDF => Some(Instruction::),
        }
    }
    
    fn handle_byte_e0_to_ef(byte: u8) -> Option<Instruction> {
        match byte {
            0xE0 => Some(Instruction::),
            0xE1 => Some(Instruction::),
            0xE2 => Some(Instruction::),
            0xE3 => Some(Instruction::),
            0xE4 => Some(Instruction::),
            0xE5 => Some(Instruction::),
            0xE6 => Some(Instruction::),
            0xE7 => Some(Instruction::),
            0xE8 => Some(Instruction::),
            0xE9 => Some(Instruction::),
            0xEA => Some(Instruction::),
            0xEB => Some(Instruction::),
            0xEC => Some(Instruction::),
            0xED => Some(Instruction::),
            0xEE => Some(Instruction::),
            0xEF => Some(Instruction::),
        }
    }
    
    fn handle_byte_f0_to_ff(byte: u8) -> Option<Instruction> {
        match byte {
            0xF0 => Some(Instruction::),
            0xF1 => Some(Instruction::),
            0xF2 => Some(Instruction::),
            0xF3 => Some(Instruction::),
            0xF4 => Some(Instruction::),
            0xF5 => Some(Instruction::),
            0xF6 => Some(Instruction::),
            0xF7 => Some(Instruction::),
            0xF8 => Some(Instruction::),
            0xF9 => Some(Instruction::),
            0xFA => Some(Instruction::),
            0xFB => Some(Instruction::),
            0xFC => Some(Instruction::),
            0xFD => Some(Instruction::),
            0xFE => Some(Instruction::),
            0xFF => Some(Instruction::),
        }
    }

}

#[derive(Debug)]
enum PrefixTarget {
    A, B, C, D, E, H, L, HL
}

#[derive(Debug)]
pub enum IncDecTarget {
    DE, A, BC, B, C
}

#[derive(Debug)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

#[derive(Debug)]
pub enum ArithmeticSource {
    BC
}

#[derive(Debug)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

#[derive(Debug)]
pub enum LoadTarget {
    A, B, C, D, E, H, L, HLI, BC, D16
}

#[derive(Debug)]
pub enum LoadSource {
    A, B, C, D, E, H , L, D8, HLI, BC, D16, SP
}

#[derive(Debug)]
pub enum StackTarget {
    BC,
}

#[derive(Debug)]
pub enum LoadType {
    // the docs were very confusing here.
    // byte: load a byte from the source to the target
    // word: load a word from the source to the target (need to change the enums here)
    // AFromIndirect, load INTO A from an indirect source 
    // IndirectFromA, load INTO the indirect address with the value from a
    // AFromByte, load into the A from the LAST memory address
    // ByteAddressFromA, load into LAST address from A
    Byte(LoadTarget, LoadSource),
    Word(LoadTarget, LoadSource),
    /* load */ AFromIndirect(LoadTarget, LoadSource),
    /* load */ IndirectFromA(LoadTarget, LoadSource),
    AFromByteAddress(LoadTarget, LoadSource),
    ByteAddressFromA(LoadTarget, LoadSource),
    ByteFromIndirect(LoadTarget, LoadSource),
    IndirectFromByte(LoadTarget, LoadSource),
}


