use sdl2::keyboard::Scancode;

pub const KEYPAD: [[u8; 4]; 4] = [
    [0x16, 0x1E, 0x26, 0x25], // 1, 2, 3, 4
    [0x15, 0x1D, 0x24, 0x2D], // Q, W, E, R
    [0x1C, 0x1B, 0x23, 0x2B], // A, S, D, F
    [0x1A, 0x22, 0x21, 0x2A], // Z, X, C, V
];

pub fn u8_to_scancode(key: u8) -> Option<Scancode> {
    match key {
        0x1 => Some(Scancode::Num1),
        0x2 => Some(Scancode::Num2),
        0x3 => Some(Scancode::Num3),
        0xC => Some(Scancode::Num4),
        0x4 => Some(Scancode::Q),
        0x5 => Some(Scancode::W),
        0x6 => Some(Scancode::E),
        0xD => Some(Scancode::R),
        0x7 => Some(Scancode::A),
        0x8 => Some(Scancode::S),
        0x9 => Some(Scancode::D),
        0xE => Some(Scancode::F),
        0xA => Some(Scancode::Z),
        0x0 => Some(Scancode::X),
        0xB => Some(Scancode::C),
        0xF => Some(Scancode::V),
        _ => None,
    }
}
