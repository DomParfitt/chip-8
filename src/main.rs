extern crate piston_window;

use piston_window::*;

mod chip8;
mod sprite;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "CHIP 8",
        [64 * 10, 32 * 10]
    ).build().unwrap();
    let mut chip8 = chip8::Chip8::new();
    chip8.memory[0x200] = 0xA2;
    chip8.memory[0x201] = 0xF0;
    println!("Value at I: {:X}", chip8.I);
    chip8.emulate_cycle();
    println!("Value at I: {:X}", chip8.I);

    chip8.graphics[224] = 1;
    chip8.print_display();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, graphics| {
            clear([0.5, 0.5, 0.5, 1.0], graphics);

            for y in 0..32 {
                for x in 0..64 {
                    let mut color = [0.0, 0.0, 0.0, 1.0];
                    if chip8.graphics[x + y * 64] != 0 {
                        color = [1.0, 1.0, 1.0, 1.0];
                    } 
                    rectangle(color, [10.0 * x as f64, 10.0 * y as f64, 10.0, 10.0], context.transform, graphics);
                }
            }

        });
    }
}
