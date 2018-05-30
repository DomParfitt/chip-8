extern crate piston_window;
extern crate rand;

use piston_window::{rectangle, WindowSettings, PistonWindow, clear};

mod chip8;
mod sprite;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "CHIP 8",
        [64 * 10, 32 * 10]
    ).build().unwrap();
    let mut chip8 = chip8::Chip8::new();
    chip8.memory[0x200] = 0xD0;
    chip8.memory[0x201] = 0x15;
    chip8.memory[0x202] = 0xFF;
    chip8.V[0] = 1;
    chip8.V[1] = 1;
    chip8.I = 0x000;
    chip8.emulate_cycle();

    // chip8.graphics[224] = true;
    // chip8.print_display();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, graphics| {
            clear([0.5, 0.5, 0.5, 1.0], graphics);

            for y in 0..32 {
                for x in 0..64 {
                    let mut color = [0.0, 0.0, 0.0, 1.0]; //BLACK
                    if chip8.pixel_at(x, y) {
                        color = [1.0, 1.0, 1.0, 1.0]; //WHITE
                    } 
                    rectangle(color, [10.0 * x as f64, 10.0 * y as f64, 10.0, 10.0], context.transform, graphics);
                }
            }

        });
    }
}
