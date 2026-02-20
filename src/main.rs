mod chip8;
mod display;
mod input;

use chip8::{Chip8, HEIGHT, WIDTH};
use display::update_minifb_buffer;
use input::map_minifbkey_to_chip_key;
use minifb::{Key, Window, WindowOptions};
use rodio::source::SineWave;
use rodio::{OutputStreamBuilder, Sink, Source};
use std::env;
use std::time::Duration;

const STEP_DEBUG: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <rom_path>", args[0]);
        std::process::exit(1);
    }

    let rom_path = &args[1];

    // Set up audio
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("open default audio stream");

    let sink = Sink::connect_new(&stream_handle.mixer());

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs(3600))
        .amplify(0.20);

    sink.append(source);
    sink.pause();

    // Setup Emulator
    let mut chip8 = Chip8::new();
    chip8.load_rom(rom_path.clone());

    let mut window = Window::new(
        "CHIP-8 Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if STEP_DEBUG {
            if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
                chip8.tick();
            }
        } else {
            // running ~10 cycles per 60Hz frame ~600Hz.
            chip8.reset_keys();
            window.get_keys().iter().for_each(|key| {
                if let Some(k) = map_minifbkey_to_chip_key(*key) {
                    chip8.keys[k as usize] = true;
                }
            });

            for _ in 0..10 {
                chip8.tick();
            }
        }

        chip8.decay_pixels();
        chip8.update_timers();

        if chip8.sound_timer > 0 {
            sink.play();
        } else {
            sink.pause();
        }

        update_minifb_buffer(&chip8.display_buffer, &mut screen_buffer);

        window
            .update_with_buffer(&screen_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
