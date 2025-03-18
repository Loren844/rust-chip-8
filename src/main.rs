mod core;
mod display;
mod gui;
mod utils;

use core::cpu::Cpu;
use display::screen::Screen;
use utils::loader::load_rom;

use std::{thread::sleep, time};
use std::io;

fn main() {
    let mut cpu: Cpu = Cpu::new();
    let mut screen: Screen = Screen::new();
    let mut path = String::new();

    //selection de la ROM
    while path.trim().is_empty() {
        println!("Saisissez le chemin d'accès vers la ROM :");
        io::stdin().read_line(&mut path).expect("Échec de la lecture de l'entrée");
        path = path.trim().to_string();

        match load_rom(&path) {
            Ok(rom) => cpu.memory.load_rom(rom),
            Err(_) => {
                eprintln!("Une erreur est survenue en chargeant la ROM !");
                path = String::new();
            },
        }
    }   
    
    //boucle infinie
    loop {
        //show the screen
        screen.draw();

        //fetch
        let instruction: u16 = cpu.read_instruction();

        //decode
        let first_nibble: u16 = instruction >> 12;
        let x = (instruction >> 8) & 0xF;
        let y = (instruction >> 4) & 0xF;
        let n = instruction & 0xF;
        let nn = (instruction & 0xFF);
        let nnn = instruction & 0xFFF;

        println!("{}", first_nibble);

        match first_nibble {
            0 => match x {
                0 => match y {
                    0xE => match n {
                        0 => screen.clear(),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            1 => cpu.program_counter = nnn,
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => cpu.v_registers[x as usize] = nn as u8,
            7 => cpu.v_registers[x as usize] += nn as u8,
            8 => {}
            9 => {}
            0xA => cpu.index_register = nnn,
            0xB => {}
            0xC => {}
            0xD => {
                let vx = cpu.v_registers[x as usize] % screen.get_width();
                let vy = cpu.v_registers[y as usize] % screen.get_height();
                cpu.v_registers[0xF] = 0;

                for i in 0..n {
                    let pattern = cpu.memory.get_instruction(cpu.index_register + i);
                    let pos_y = vy + i as u8;
                    for j in 0..8 {
                        let bit = pattern >> (7 - j) & 1; // de gauche a droite
                        let pos_x = vx + j;
                        if pos_x > screen.get_width() {
                            break;
                        }
                        if bit == 1 {
                            screen.swap(pos_x as usize, pos_y as usize);
                            if screen.get_pixel(pos_x as usize, pos_y as usize) == true { cpu.v_registers[0xF] = 1 }
                        }
                    }
                    if pos_y + 1 > screen.get_height() {
                        break;
                    }
                }
            }
            0xE => {}
            0xF => {}
            _ => {}
        }
        sleep(time::Duration::from_millis(100))
    }
    
}
