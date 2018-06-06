extern crate piston;
extern crate piston_window;
extern crate rand;

use piston::input::*;
use piston_window::{clear, rectangle, PistonWindow, WindowSettings};
use std::time::{Duration, Instant};

mod assembler;
mod chip8;
mod sprite;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("CHIP 8", [64 * 10, 32 * 10])
        .build()
        .unwrap();
    let mut chip8 = chip8::Chip8::new();
    chip8.load("./roms/pong".to_string());

    // for i in 0..12 {
    //     //Set origin
    //     chip8.memory[0x200 + i * 8] = 0x60;
    //     chip8.memory[0x201 + i * 8] = 0x32 + (5 * i as u8) + 1;
    //     chip8.memory[0x202 + i * 8] = 0x61;
    //     chip8.memory[0x203 + i * 8] = 0x01;

    //     //Load 0x000 into I
    //     chip8.memory[0x204 + i * 8] = 0xA0;
    //     chip8.memory[0x205 + i * 8] = 0x00 + 5 * i as u8;

    //     //Draw
    //     chip8.memory[0x206 + i * 8] = 0xD0;
    //     chip8.memory[0x207 + i * 8] = 0x15;
    // }

    // chip8.graphics[224] = true;
    // chip8.print_display();

    let mut now = Instant::now();
    let mut cycle_count = 0;
    while let Some(e) = window.next() {
        chip8.emulate_cycle();
        cycle_count += 1;
        // if cycle_count % 60 == 0 {
            // println!(
            //     "Cycle # {}. {}.{} seconds since last cycle.",
            //     cycle_count,
            //     now.elapsed().as_secs(),
            //     now.elapsed().subsec_nanos()
            // );
            // now = Instant::now();
        // }

        if let Some(Button::Keyboard(key_pressed)) = e.press_args() {
            println!("Key pressed {:?}", key_pressed);
            match key_pressed {
                Key::NumPad1 => {
                    chip8.keyboard[0] = true;
                }
                Key::NumPad2 => {
                    chip8.keyboard[1] = true;
                }
                Key::NumPad3 => {
                    chip8.keyboard[2] = true;
                }
                Key::NumPad4 => {
                    chip8.keyboard[3] = true;
                }
                Key::Q => {
                    chip8.keyboard[4] = true;
                }
                Key::W => {
                    chip8.keyboard[5] = true;
                }
                Key::E => {
                    chip8.keyboard[6] = true;
                }
                Key::R => {
                    chip8.keyboard[7] = true;
                }
                Key::A => {
                    chip8.keyboard[8] = true;
                }
                Key::S => {
                    chip8.keyboard[9] = true;
                }
                Key::D => {
                    chip8.keyboard[10] = true;
                }
                Key::F => {
                    chip8.keyboard[11] = true;
                }
                Key::Z => {
                    chip8.keyboard[12] = true;
                }
                Key::X => {
                    chip8.keyboard[13] = true;
                }
                Key::C => {
                    chip8.keyboard[14] = true;
                }
                Key::V => {
                    chip8.keyboard[15] = true;
                }
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key_released)) = e.release_args() {
            match key_released {
                Key::NumPad1 => {
                    chip8.keyboard[0] = false;
                }
                Key::NumPad2 => {
                    chip8.keyboard[1] = false;
                }
                Key::NumPad3 => {
                    chip8.keyboard[2] = false;
                }
                Key::NumPad4 => {
                    chip8.keyboard[3] = false;
                }
                Key::Q => {
                    chip8.keyboard[4] = false;
                }
                Key::W => {
                    chip8.keyboard[5] = false;
                }
                Key::E => {
                    chip8.keyboard[6] = false;
                }
                Key::R => {
                    chip8.keyboard[7] = false;
                }
                Key::A => {
                    chip8.keyboard[8] = false;
                }
                Key::S => {
                    chip8.keyboard[9] = false;
                }
                Key::D => {
                    chip8.keyboard[10] = false;
                }
                Key::F => {
                    chip8.keyboard[11] = false;
                }
                Key::Z => {
                    chip8.keyboard[12] = false;
                }
                Key::X => {
                    chip8.keyboard[13] = false;
                }
                Key::C => {
                    chip8.keyboard[14] = false;
                }
                Key::V => {
                    chip8.keyboard[15] = false;
                }
                _ => {}
            }
        }

        if chip8.V[0xF] != 0 {
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
}
