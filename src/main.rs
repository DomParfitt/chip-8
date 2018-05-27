mod chip8;
mod sprite;

fn main() {
    println!("Hello, world!");
    let mut chip8 = chip8::Chip8::new();
    chip8.memory[0x200] = 0xA2;
    chip8.memory[0x201] = 0xF0;
    chip8.emulate_cycle();
}
