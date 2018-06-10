use chip8::Chip8;

pub struct Opcode {
    pub high_byte: u8,
    pub low_byte: u8,
    pub instruction: u8,
    pub x: usize,
    pub y: usize,
    pub n: usize,
    pub nnn: u16,
}

impl From<u16> for Opcode {
    fn from(bytes: u16) -> Self {
        let (high_byte, low_byte) = Chip8::bytes_from_opcode(bytes);
        Opcode {
            high_byte,
            low_byte,
            instruction: (high_byte & 0xF0) >> 4,
            x: (high_byte & 0x0F) as usize,
            y: ((low_byte & 0xF0) >> 4) as usize,
            n: (low_byte & 0x0F) as usize,
            nnn: bytes & 0x0FFF,
        }
    }
}

impl From<(u8, u8)> for Opcode {
    fn from(bytes: (u8, u8)) -> Self {
        let (high_byte, low_byte) = bytes;
        Opcode {
            high_byte,
            low_byte,
            instruction: (high_byte & 0xF0) >> 4,
            x: (high_byte & 0x0F) as usize,
            y: ((low_byte & 0xF0) >> 4) as usize,
            n: (low_byte & 0x0F) as usize,
            nnn: 0//opcode & 0x0FFF,
        }
    }
}
