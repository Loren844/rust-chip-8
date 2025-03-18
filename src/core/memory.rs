use crate::display::font::FONT;

pub struct Memory {
    pub ram: [u8; 4096], //écrire a partir de 200 (0x16)
}

impl Memory {
    pub fn new() -> Self {
        let mut ram: [u8; 4096] = [0; 4096];
        ram[0x050..0x0A0].copy_from_slice(&FONT);
        Memory { ram }
    }

    pub fn get_instruction(&mut self, pc: u16) -> u8 {
        return self.ram[pc as usize];
    }
}
