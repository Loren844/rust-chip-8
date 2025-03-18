use crate::core::memory::Memory;
use crate::core::screen::Screen;
use crate::core::timers::Timers;

pub struct Chip8 {
    pub memory: Memory,
    pub screen: Screen,
    pub timers: Timers,
    pub program_counter: u16,
    pub stack: [u16; 16],
    pub index_register: u16,
    pub v_registers: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Memory::new(),
            screen: Screen::new(),
            timers: Timers::new(),
            program_counter: 0x200,
            stack: [0; 16],
            index_register: 0,
            v_registers: [0; 16],
        }
    }

    pub fn read_instruction(&mut self) -> u16 {
        let b1 = self.memory.get_instruction(self.program_counter);
        let b2 = self.memory.get_instruction(self.program_counter + 1);

        self.program_counter += 2;
        return ((b1 as u16) << 8) | (b2 as u16);
    }

    pub fn decode_and_execute(&mut self, instruction: u16) {
        let first_nibble: u16 = instruction >> 12;
        let x = (instruction >> 8) & 0xF;
        let y = (instruction >> 4) & 0xF;
        let n = instruction & 0xF;
        let nn = instruction & 0xFF;
        let nnn = instruction & 0xFFF;

        match first_nibble {
            0 => match x {
                0 => match y {
                    0xE => match n {
                        0 => self.screen.clear(),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            1 => self.program_counter = nnn,
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => self.v_registers[x as usize] = nn as u8,
            7 => self.v_registers[x as usize] += nn as u8,
            8 => {}
            9 => {}
            0xA => self.index_register = nnn,
            0xB => {}
            0xC => {}
            0xD => {
                let vx = self.v_registers[x as usize] % self.screen.get_width();
                let vy = self.v_registers[y as usize] % self.screen.get_height();
                self.v_registers[0xF] = 0;

                for i in 0..n {
                    let pattern = self.memory.get_instruction(self.index_register + i);
                    let pos_y = vy + i as u8;
                    for j in 0..8 {
                        let bit = pattern >> (7 - j) & 1; // de gauche a droite
                        let pos_x = vx + j;
                        if pos_x > self.screen.get_width() {
                            break;
                        }
                        if bit == 1 {
                            self.screen.swap(pos_x as usize, pos_y as usize);
                            if self.screen.get_pixel(pos_x as usize, pos_y as usize) == true {
                                self.v_registers[0xF] = 1
                            }
                        }
                    }
                    if pos_y + 1 > self.screen.get_height() {
                        break;
                    }
                }
            }
            0xE => {}
            0xF => {}
            _ => {}
        }
    }
}
