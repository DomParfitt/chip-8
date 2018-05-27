pub struct Chip8 {
    pub memory: [u8; 4096],
    pub V: [u8; 16],
    pub I: u16,
    pc: usize,
    sp: usize,
    pub delay: u8,
    pub sound: u8,
    pub stack: [u16; 16],
    pub keyboard: [u8; 16],
    pub graphics: [u8; 64 * 32],

}

impl Chip8 {
    pub fn new() -> Self {
        Chip8{
            memory: [0; 4096],
            V: [0; 16],
            I: 0,
            pc: 0x200,
            sp: 0,
            delay: 0,
            sound: 0,
            stack: [0; 16],
            keyboard: [0; 16],
            graphics: [0; 64 * 32],
        }
    }

    pub fn emulate_cycle(&mut self) {
        let mut opcode: u16 = self.fetch_opcode();
        println!("Opcode: {:04X}", opcode);
        self.decode_opcode(opcode);
        self.execute_opcode(opcode);
    }

    fn fetch_opcode(&self) -> u16 {
        let high_order: u16 = u16::from(self.memory[self.pc])  << 8;
        println!("High Order Byte: {:04X}", high_order);
        let low_order: u16 = u16::from(self.memory[self.pc + 1]);
        println!("Low Order Byte: {:04X}", low_order);
        high_order | low_order
    }

    fn decode_opcode(&mut self, opcode: u16) {
        let (high_byte, low_byte) = Chip8::bytes_from_opcode(opcode);
        let instruction: u8 = (high_byte & 0xF0) >> 4;
        let x :usize = (high_byte & 0x0F) as usize;
        let y: usize = (low_byte & 0xF0) as usize;
        let nnn: u16 = opcode & 0x0FFF;
        println!("Instruction: {:X}", instruction);
        match instruction {
            0x0 => {
                match low_byte {
                    0xE0 => {
                        //Clear display
                    },
                    0xEE => {
                        //Return from subroutine
                    },
                    _ => {
                        //Jump to routine
                    }
                }
            },
            0x1 => {
                //Jump to address
                self.pc = usize::from(nnn);
            },
            0x2 => {
                //Call subroutine at address
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
                self.pc = usize::from(nnn);
            },
            0x3 => {
                //Conditional next instruction skip
                if self.V[x] == low_byte {
                    self.pc += 2;
                } 
            },
            0x4 => {
                if self.V[x] != low_byte {
                    self.pc += 2;
                }
                
            },
            0x5 => {
                if self.V[x] == self.V[y] {
                    self.pc += 2;
                }
            },
            0x6 => {
                self.V[x] = low_byte;
            },
            0x7 => {
                self.V[x] += low_byte; 
            },
            0x8 => {
                match low_byte & 0x0F {
                    0x0 => {
                        self.V[x] = self.V[y];
                    },
                    0x1 => {
                        self.V[x] = self.V[x] | self.V[y];
                    },
                    0x2 => {
                        self.V[x] = self.V[x] & self.V[y];
                    },
                    0x3 => {
                        self.V[x] = self.V[x] ^ self.V[y];
                    },
                    0x4 => {
                        let result: u16 = u16::from(self.V[x] + self.V[y]);
                        let mut vf: u8 = 0;
                        if result > 0xFF {
                            vf = 1;
                        } 
                        self.V[0xF as usize] = vf;
                        self.V[x] += (result & 0x00FF) as u8;
                    },
                    0x5 => {
                        let mut vf: u8 = 0;
                        if self.V[x] > self.V[y] {
                            vf = 1;
                        }
                        self.V[0xF as usize] = vf;
                        self.V[x] -= self.V[y]; //Should this be conditional?
                    },
                    0x6 => {
                        let mut vf: u8 = 0;
                        if self.V[x] & 0x1 == 1 {
                            vf = 1;
                        }
                        self.V[0xF as usize] = vf;
                        self.V[x] = self.V[x] >> 1;
                    },
                    0x7 => {},
                    0xE => {},
                    _ => {}
                }
            },
            0x9 => {},
            0xA => {},
            0xB => {},
            0xC => {},
            0xD => {},
            0xE => {},
            0xF => {},
            _ => {}
        }
    }

    fn execute_opcode(&mut self, opcode: u16) {

    }

    fn bytes_from_opcode(opcode: u16) -> (u8, u8) {
        let high_order: u8 = ((opcode & 0xFF00) >> 8) as u8;
        println!("High Order: {:X}", high_order);
        let low_order: u8 =  (opcode & 0x00FF) as u8;
        println!("Low Order: {:X}", low_order);
        (high_order, low_order)
    }

    fn high_nibble_from_byte(byte: u8) -> u8 {
        byte & 0xF0
    }

    fn low_nibble_from_byte(byte: u8) -> u8 {
        byte & 0x0F
    }
}