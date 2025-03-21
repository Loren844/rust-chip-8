use crate::core::keypad::u8_to_scancode;
use crate::core::memory::Memory;
use crate::core::screen::Screen;
use crate::core::stack::Stack;
use crate::core::timers::Timers;

use rand::Rng;
use sdl2::EventPump;

pub struct Chip8 {
    pub memory: Memory,
    pub screen: Screen,
    pub timers: Timers,
    pub program_counter: u16,
    pub stack: Stack,
    pub index_register: u16,
    pub v_registers: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Memory::new(),
            screen: Screen::new(),
            stack: Stack::new(),
            timers: Timers::new(),
            program_counter: 0x200,
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

    pub fn decode_and_execute(&mut self, instruction: u16, event_pump: &EventPump) {
        let keys = event_pump.keyboard_state();
        //decode
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
                        0 => self.screen.clear(),                       //00E0
                        0xE => self.program_counter = self.stack.pop(), //00EE
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            1 => self.program_counter = nnn,
            2 => {
                self.stack.push(self.program_counter);
                self.program_counter = nnn;
            }
            3 => {
                if self.v_registers[x as usize] as u16 == nn {
                    self.program_counter += 2
                }
            }
            4 => {
                if self.v_registers[x as usize] as u16 != nn {
                    self.program_counter += 2
                }
            }
            5 => {
                if self.v_registers[x as usize] == self.v_registers[y as usize] {
                    self.program_counter += 2
                }
            }
            6 => self.v_registers[x as usize] = nn as u8,
            7 => self.v_registers[x as usize] = self.v_registers[x as usize].wrapping_add(nn as u8),
            8 => {
                match n {
                    0 => self.v_registers[x as usize] = self.v_registers[y as usize],
                    1 => {
                        self.v_registers[x as usize] =
                            self.v_registers[x as usize] | self.v_registers[y as usize]
                    }
                    2 => {
                        self.v_registers[x as usize] =
                            self.v_registers[x as usize] & self.v_registers[y as usize]
                    }
                    3 => {
                        self.v_registers[x as usize] =
                            self.v_registers[x as usize] ^ self.v_registers[y as usize]
                    }
                    4 => {
                        self.v_registers[x as usize] =
                            self.v_registers[x as usize].wrapping_add(self.v_registers[y as usize]);
                        if self.v_registers[x as usize] >= 255 {
                            self.v_registers[0xF] = 1
                        } else {
                            self.v_registers[0xF] = 0
                        }
                    }
                    5 => {
                        if self.v_registers[x as usize] >= self.v_registers[y as usize] {
                            self.v_registers[0xF] = 1;
                        } else {
                            self.v_registers[0xF] = 0;
                        }
                        self.v_registers[x as usize] =
                            self.v_registers[x as usize].wrapping_sub(self.v_registers[y as usize]);
                    }
                    6 => {
                        //self.v_registers[x as usize] = self.v_registers[y as usize]; COSMAC VIP
                        let s_bit = self.v_registers[x as usize] & 1;
                        self.v_registers[x as usize] = self.v_registers[x as usize] >> 1;
                        self.v_registers[0xF] = s_bit;
                    }
                    7 => {
                        if self.v_registers[x as usize] > self.v_registers[y as usize] {
                            self.v_registers[0xF] = 0
                        } else if self.v_registers[x as usize] <= self.v_registers[y as usize] {
                            self.v_registers[0xF] = 1
                        }
                        self.v_registers[x as usize] =
                            self.v_registers[y as usize].wrapping_sub(self.v_registers[x as usize]);
                    }
                    0xE => {
                        //self.v_registers[x as usize] = self.v_registers[y as usize]; COSMAC VIP
                        let s_bit = (self.v_registers[x as usize] >> 7) & 1;
                        self.v_registers[x as usize] = self.v_registers[x as usize] << 1;
                        self.v_registers[0xF] = s_bit;
                    }
                    _ => {}
                }
            }
            9 => {
                if self.v_registers[x as usize] != self.v_registers[y as usize] {
                    self.program_counter += 2
                }
            }
            0xA => self.index_register = nnn,
            0xB => {
                //self.program_counter = nnn + self.v_registers[0] as u16; COSMAC VIP
                self.program_counter = nnn + self.v_registers[x as usize] as u16;
            }
            0xC => {
                let rand = rand::rng().random_range(0..nn);
                self.v_registers[x as usize] = (rand & nn) as u8;
            }
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
            0xE => {
                let key = u8_to_scancode(self.v_registers[x as usize]).unwrap();
                match y {
                    9 => {
                        if keys.is_scancode_pressed(key) {
                            self.program_counter += 2;
                        }
                    }
                    0xA => {
                        if keys.is_scancode_pressed(key) {
                            self.program_counter += 2;
                        }
                    }
                    _ => {}
                }
            }
            0xF => match nn {
                0x07 => self.v_registers[x as usize] = self.timers.delay_timer,
                0x15 => self.timers.delay_timer = self.v_registers[x as usize],
                0x18 => self.timers.sound_timer = self.v_registers[x as usize],
                0x1E => self.index_register += self.v_registers[x as usize] as u16,
                0x0A => {
                    if let Some(scancode) = keys.pressed_scancodes().next() {
                        self.v_registers[x as usize] = scancode as u8;
                    } else {
                        self.program_counter -= 2;
                    }
                }
                0x29 => self.index_register = 0x050 + (self.v_registers[x as usize] * 5) as u16,
                0x33 => {
                    let num = self.v_registers[x as usize];
                    self.memory.ram[self.index_register as usize] = num / 100;
                    self.memory.ram[(self.index_register + 1) as usize] = num % 100 / 10;
                    self.memory.ram[(self.index_register + 2) as usize] = num % 10;
                }
                0x55 => {
                    for i in 0..x + 1 {
                        self.memory.ram[(self.index_register + i) as usize] =
                            self.v_registers[i as usize]
                        //self.index_register += 1 COSMAC VIP et enlever le + i evidemment
                    }
                }
                0x65 => {
                    for i in 0..x + 1 {
                        self.v_registers[i as usize] =
                            self.memory.ram[(self.index_register + i) as usize];
                        //self.index_register += 1 COSMAC VIP et enlever le + i evidemment
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
