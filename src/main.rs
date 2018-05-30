extern crate piston_window;
extern crate rand;

use piston_window::{clear, rectangle, PistonWindow, WindowSettings};

mod chip8;
mod sprite;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("CHIP 8", [64 * 10, 32 * 10])
        .build()
        .unwrap();
    let mut chip8 = chip8::Chip8::new();

    for i in 0..12 {
        //Set origin
        chip8.memory[0x200 + i * 8] = 0x60;
        chip8.memory[0x201 + i * 8] = 0x32 + (5 * i as u8) + 1;
        chip8.memory[0x202 + i * 8] = 0x61;
        chip8.memory[0x203 + i * 8] = 0x01;

        //Load 0x000 into I
        chip8.memory[0x204 + i * 8] = 0xA0;
        chip8.memory[0x205 + i * 8] = 0x00 + 5 * i as u8;

        //Draw
        chip8.memory[0x206 + i * 8] = 0xD0;
        chip8.memory[0x207 + i * 8] = 0x15;
    }

    // chip8.graphics[224] = true;
    // chip8.print_display();

    while let Some(e) = window.next() {
        chip8.emulate_cycle();
        window.draw_2d(&e, |context, graphics| {
            clear([0.5, 0.5, 0.5, 1.0], graphics);

            for y in 0..32 {
                for x in 0..64 {
                    let mut color = [0.0, 0.0, 0.0, 1.0]; //BLACK
                    if chip8.pixel_at(x, y) {
                        color = [1.0, 1.0, 1.0, 1.0]; //WHITE
                    }
                    rectangle(
                        color,
                        [10.0 * x as f64, 10.0 * y as f64, 10.0, 10.0],
                        context.transform,
                        graphics,
                    );
                }
            }
        });
    }
}
