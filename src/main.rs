mod chip8;
mod sprite;

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.memory[0x200] = 0xA2;
    chip8.memory[0x201] = 0xF0;
    println!("Value at I: {:X}", chip8.I);
    chip8.emulate_cycle();
    println!("Value at I: {:X}", chip8.I);
    chip8.print_display();
}
