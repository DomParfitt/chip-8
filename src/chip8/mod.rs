#[cfg(test)]
mod tests;

extern crate rand;
use rand::prelude::*;

use sprite;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub V: [u8; 16],
    pub I: u16,
    pc: usize,
    sp: usize,
    pub delay: u8,
    pub sound: u8,
    pub stack: [u16; 16],
    pub keyboard: [bool; 16],
    pub graphics: [bool; WIDTH * HEIGHT],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut cpu = Chip8 {
            memory: [0; 4096],
            V: [0; 16],
            I: 0,
            pc: 0x200,
            sp: 0,
            delay: 0,
            sound: 0,
            stack: [0; 16],
            keyboard: [false; 16],
            graphics: [false; WIDTH * HEIGHT],
        };
        cpu.init();
        cpu
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        for i in 0..program.len() {
            if i > 0x1000 {
                panic!("Program is too large for memory.");
            }

            self.memory[0x200 + i] = program.as_slice()[i];
        }
    }

    pub fn load(&mut self, rom: String) {}

    pub fn print_display(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", if self.pixel_at(x, y) { 1 } else { 0 });
            }
            println!("");
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode: u16 = self.fetch_opcode();
        // println!("Opcode: {:04X}", opcode);
        self.decode_opcode(opcode);
        self.pc += 2;
        if self.pc >= 0x1000 {
            self.pc = 0x200;
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> bool {
        self.graphics[x + y * WIDTH]
    }

    pub fn pixel_byte_at(&self, x: usize, y: usize) -> [bool; 8] {
        let mut pixel_byte: [bool; 8] = [false; 8];
        for i in 0..8 {
            let x_shifted = (x + i) % WIDTH;
            pixel_byte[i] = self.pixel_at(x_shifted, y);
        }
        pixel_byte
    }
}

///Private functions
impl Chip8 {
    fn init(&mut self) {
        let fonts = sprite::get_font_set();
        for i in 0..fonts.len() {
            for byte in 0..fonts[i].len() {
                self.memory[byte + i * fonts[i].len()] = fonts[i][byte];
            }
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let high_order: u16 = u16::from(self.memory[self.pc]) << 8;
        // println!("High Order Byte: {:04X}", high_order);
        let low_order: u16 = u16::from(self.memory[self.pc + 1]);
        // println!("Low Order Byte: {:04X}", low_order);
        high_order | low_order
    }

    fn decode_opcode(&mut self, opcode: u16) {
        let (high_byte, low_byte) = Chip8::bytes_from_opcode(opcode);
        let instruction: u8 = (high_byte & 0xF0) >> 4;
        let x: usize = (high_byte & 0x0F) as usize;
        let y: usize = ((low_byte & 0xF0) >> 4) as usize;
        let n: usize = (low_byte & 0x0F) as usize;
        let nnn: u16 = opcode & 0x0FFF;
        // println!("Instruction: {:X}\nX: {:X}\nY: {:X}\nN: {:X}\nNNN: {:X}", instruction, x, y, n, nnn);
        match instruction {
            0x0 => {
                match low_byte {
                    0xE0 => {
                        //Clear display
                        for y in 0..HEIGHT {
                            for x in 0..WIDTH {
                                self.graphics[x + y * 64] = false;
                            }
                        }
                    }
                    0xEE => {
                        //Return from subroutine
                        self.pc = usize::from(self.stack[self.sp]);
                        self.sp -= 1;
                    }
                    _ => {
                        //Jump to routine
                    }
                }
            }
            0x1 => {
                //Jump to address
                self.pc = usize::from(nnn);
            }
            0x2 => {
                //Call subroutine at address
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
                self.pc = usize::from(nnn);
            }
            0x3 => {
                //Conditional next instruction skip
                if self.V[x] == low_byte {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.V[x] != low_byte {
                    self.pc += 2;
                }
            }
            0x5 => {
                if self.V[x] == self.V[y] {
                    self.pc += 2;
                }
            }
            0x6 => {
                self.V[x] = low_byte;
            }
            0x7 => {
                self.V[x] += low_byte;
            }
            0x8 => {
                match n {
                    0x0 => {
                        self.V[x] = self.V[y];
                    }
                    0x1 => {
                        self.V[x] = self.V[x] | self.V[y];
                    }
                    0x2 => {
                        self.V[x] = self.V[x] & self.V[y];
                    }
                    0x3 => {
                        self.V[x] = self.V[x] ^ self.V[y];
                    }
                    0x4 => {
                        let result: u16 = u16::from(self.V[x] + self.V[y]);
                        let mut vf: u8 = 0;
                        if result > 0xFF {
                            vf = 1;
                        }
                        self.V[0xF as usize] = vf;
                        self.V[x] += (result & 0x00FF) as u8;
                    }
                    0x5 => {
                        let mut vf: u8 = 0;
                        if self.V[x] > self.V[y] {
                            vf = 1;
                        }
                        self.V[0xF as usize] = vf;
                        self.V[x] -= self.V[y]; //Should this be conditional?
                    }
                    0x6 => {
                        let mut vf: u8 = 0;
                        if self.V[x] & 0x1 == 1 {
                            vf = 1;
                        }
                        self.V[0xF as usize] = vf;
                        self.V[x] = self.V[x] >> 1;
                    }
                    0x7 => {
                        let mut vf: u8 = 0;
                        if self.V[y] > self.V[x] {
                            vf = 1;
                        }
                        self.V[0xF as usize] = vf;
                        self.V[x] = self.V[y] - self.V[x]; //Should this be conditional?
                    }
                    0xE => {
                        let mut vf: u8 = 0;
                        if self.V[x] & 0x80 == 1 {
                            vf = 1;
                        }
                        self.V[0xF as usize] = vf;
                        self.V[x] = self.V[x] << 1;
                    }
                    _ => {}
                }
            }
            0x9 => {
                if self.V[x] != self.V[y] {
                    self.pc += 2;
                }
            }
            0xA => {
                self.I = nnn;
            }
            0xB => {
                self.pc = usize::from(u16::from(self.V[0]) + nnn);
            }
            0xC => {
                //Random
                let random_byte: u8 = random();
                self.V[x] = random_byte & low_byte;
            }
            0xD => {
                let (x, y) = (usize::from(self.V[x]), usize::from(self.V[y]));
                for i in 0..n {
                    let byte: u8 = self.memory[usize::from(self.I) + i];
                    // println!("New display byte {:b}", byte);
                    let pixels: u8 = Chip8::byte_from_bool_array(self.pixel_byte_at(x, y + i));
                    // println!("Current pixels {:b}", pixels);
                    let new_pixels = byte ^ pixels;
                    // println!("New pixels {:b}", new_pixels);
                    if pixels != new_pixels {
                        self.V[0xF] = 1;
                    } else {
                        self.V[0xF] = 0;
                    }
                    self.update_pixels_at(x, y + i, Chip8::bool_array_from_byte(new_pixels));
                }
            }
            0xE => {
                let key = self.V[x] as usize;
                match low_byte {
                    0x9E => {
                        if self.keyboard[key] {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        if !self.keyboard[key] {
                            self.pc += 2;
                        }
                    }
                    _ => {}
                }
            }
            0xF => match low_byte {
                0x07 => {
                    self.V[x] = self.delay;
                }
                0x0A => {
                    let mut key_pressed = false;
                    for i in 0..self.keyboard.len() {
                        if self.keyboard[i] {
                            self.V[x] = i as u8;
                            key_pressed = true;
                            break;
                        }
                    }
                    if !key_pressed {
                        self.pc -= 2;
                    }
                }
                0x15 => {
                    self.delay = self.V[x];
                }
                0x18 => {
                    self.sound = self.V[x];
                }
                0x1E => {
                    self.I += u16::from(self.V[x]);
                }
                0x29 => {
                    let character = u16::from(self.V[x]);
                    self.I = character * 5;
                }
                0x33 => {
                    let value = self.V[x];
                    let index = self.I as usize;
                    let hundreds = value / 100;
                    let tens = (value - (hundreds * 100)) / 10;
                    let units = value - (hundreds * 100) - (tens * 10);
                    self.memory[index] = hundreds;
                    self.memory[index + 1] = tens;
                    self.memory[index + 2] = units;
                }
                0x55 => {
                    for i in 0..x {
                        self.memory[usize::from(self.I) + i] = self.V[i];
                    }
                }
                0x65 => {
                    for i in 0..x {
                        self.V[i] = self.memory[usize::from(self.I) + i];
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn bytes_from_opcode(opcode: u16) -> (u8, u8) {
        let high_order: u8 = ((opcode & 0xFF00) >> 8) as u8;
        // println!("High Order: {:X}", high_order);
        let low_order: u8 = (opcode & 0x00FF) as u8;
        // println!("Low Order: {:X}", low_order);
        (high_order, low_order)
    }

    fn high_nibble_from_byte(byte: u8) -> u8 {
        byte & 0xF0
    }

    fn low_nibble_from_byte(byte: u8) -> u8 {
        byte & 0x0F
    }

    fn byte_from_bool_array(array: [bool; 8]) -> u8 {
        let mut result: u8 = 0b00000000;
        for i in 0..8 {
            if array[i] {
                result = result | (0b10000000 >> i)
            }
        }
        result
    }

    fn bool_array_from_byte(byte: u8) -> [bool; 8] {
        let mut array: [bool; 8] = [false; 8];
        for i in 0..8 {
            let temp_byte = byte;
            array[i] = temp_byte & (0b10000000 >> i) != 0;
        }
        array
    }

    fn update_pixel_at(&mut self, x: usize, y: usize, pixel: bool) {
        self.graphics[x + y * WIDTH] = pixel;
    }

    fn update_pixels_at(&mut self, x: usize, y: usize, pixels: [bool; 8]) {
        for i in 0..8 {
            let x_shifted = (x + i) % WIDTH;
            self.update_pixel_at(x_shifted, y, pixels[i]);
        }
    }
}
