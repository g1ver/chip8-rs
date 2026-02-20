use crate::chip8::{HEIGHT, WIDTH};

pub fn update_minifb_buffer(chip8_buffer: &[u8; HEIGHT * WIDTH], minifb_buffer: &mut [u32]) {
    for i in 0..(HEIGHT * WIDTH) {
        let brightness = chip8_buffer[i];

        let fg = 0xE5CC80;
        let bg = 0x333333;

        minifb_buffer[i] = blend_colors(bg, fg, brightness);
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
