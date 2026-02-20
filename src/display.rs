use crate::chip8::{HEIGHT, WIDTH};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

pub struct Display {
    window: Window,
    screen_buffer: Vec<u32>,
    bg_color: u32,
    fg_color: u32,
}

impl Display {
    pub fn new(title: &str, scale: Scale) -> Self {
        let window = Window::new(
            title,
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| panic!("{}", e));

        Self {
            window,
            screen_buffer: vec![0u32; WIDTH * HEIGHT],
            bg_color: 0x333333,
            fg_color: 0xE5CC80,
        }
    }

    pub fn set_target_fps(&mut self, fps: usize) {
        self.window.set_target_fps(fps);
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.window.is_key_pressed(key, KeyRepeat::No)
    }

    pub fn get_keys(&self) -> Vec<Key> {
        self.window.get_keys()
    }

    pub fn draw(&mut self, chip8_buffer: &[u8; HEIGHT * WIDTH]) {
        for i in 0..(HEIGHT * WIDTH) {
            self.screen_buffer[i] = blend_colors(self.bg_color, self.fg_color, chip8_buffer[i]);
        }

        self.window
            .update_with_buffer(&self.screen_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn blend_colors(bg: u32, fg: u32, alpha: u8) -> u32 {
    let alpha = alpha as u32;

    // Extract RGB components
    let bg_r = (bg >> 16) & 0xFF;
    let bg_g = (bg >> 8) & 0xFF;
    let bg_b = bg & 0xFF;

    let fg_r = (fg >> 16) & 0xFF;
    let fg_g = (fg >> 8) & 0xFF;
    let fg_b = fg & 0xFF;

    // Blend: bg + (fg - bg) * (alpha / 255)
    let r = bg_r + ((fg_r as i32 - bg_r as i32) * alpha as i32 / 255) as u32;
    let g = bg_g + ((fg_g as i32 - bg_g as i32) * alpha as i32 / 255) as u32;
    let b = bg_b + ((fg_b as i32 - bg_b as i32) * alpha as i32 / 255) as u32;

    (r << 16) | (g << 8) | b
}
