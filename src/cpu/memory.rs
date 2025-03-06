use crate::display::font::FONT;

pub struct Memory {
    pub ram: [u8; 4096], //écrire a partir de 200 (0x16)
    pub pc: u16,
    pub i_reg: u16
}

impl Memory {
    pub fn new() -> Self {
        let mut ram:[u8; 4096] = [0; 4096];
        ram[0x050..0x0A0].copy_from_slice(&FONT);

        Memory {
            ram,
            pc: 0x200,
            i_reg: 0,
        }
    }
}

