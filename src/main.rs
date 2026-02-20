mod audio;
mod chip8;
mod display;
mod input;

use audio::Audio;
use chip8::Chip8;
use display::Display;
use minifb::Key;
use std::env;

use crate::input::map_minifbkey_to_chip_key;

const STEP_DEBUG: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rom_path>", args[0]);
        std::process::exit(1);
    }

    let mut chip8 = Chip8::new();
    chip8.load_rom(args[1].clone());

    let mut display = Display::new("CHIP-8 Emulator", minifb::Scale::X16);
    display.set_target_fps(60);

    let mut audio = Audio::new(440.0, 0.20);

    while display.is_open() {
        if STEP_DEBUG {
            if display.is_key_pressed(Key::Space) {
                chip8.tick();
            }
        } else {
            chip8.reset_keys();
            for key in display.get_keys() {
                if let Some(k) = map_minifbkey_to_chip_key(key) {
                    chip8.keys[k as usize] = true;
                }
            }
            for _ in 0..10 {
                chip8.tick();
            }
        }

        chip8.decay_pixels();
        chip8.update_timers();
        audio.update(chip8.sound_timer);
        display.draw(&chip8.display_buffer);
    }
}
