extern crate piston_window;

use piston_window::*;

mod chip8;
mod sprite;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "CHIP 8",
        [64 * 100, 32 * 100]
    ).build().unwrap();
    let mut chip8 = chip8::Chip8::new();
    chip8.memory[0x200] = 0xA2;
    chip8.memory[0x201] = 0xF0;
    println!("Value at I: {:X}", chip8.I);
    chip8.emulate_cycle();
    println!("Value at I: {:X}", chip8.I);
    chip8.print_display();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, graphics| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            for i in 0..64 {
                for j in 0..32 {
                    let mut color = [0.0, 0.0, 0.0, 1.0];
                    if chip8.graphics[i * j + j] == 0 {
                        color = [1.0, 1.0, 1.0, 1.0];
                    } 
                    rectangle(color, [(i*j) as f64, j as f64, 100.0, 100.0], context.transform, graphics);
                }
            }

        });
    }
}
