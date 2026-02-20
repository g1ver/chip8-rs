use minifb::Key;

pub fn map_minifbkey_to_chip_key(mfbk: minifb::Key) -> Option<u8> {
    match mfbk {
        Key::Key1 => Some(0x1),
        Key::Key2 => Some(0x2),
        Key::Key3 => Some(0x3),
        Key::Key4 => Some(0xC),
        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0xD),
        Key::A => Some(0x7),
        Key::S => Some(0x8),
        Key::D => Some(0x9),
        Key::F => Some(0xE),
        Key::Z => Some(0xA),
        Key::X => Some(0x0),
        Key::C => Some(0xB),
        Key::V => Some(0xF),
        _ => None,
    }
}
