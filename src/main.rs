mod core;
mod display;
mod gui;
mod utils;

use core::cpu::Cpu;
use display::screen::Screen;
use utils::loader::load_rom;

use std::{thread::sleep, time};
use std::io;
use rand::Rng;

fn main() {
    let mut cpu: Cpu = Cpu::new();
    let mut screen: Screen = Screen::new();
    let mut path = String::new();

    print!("\x1B[2J\x1B[1;1H");

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
        print!("\x1B[2J\x1B[1;1H");
        screen.draw();

        //fetch
        let instruction: u16 = cpu.read_instruction();

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
                        0 => screen.clear(), //00E0
                        0xE => cpu.program_counter = cpu.stack.pop(), //00EE
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
            1 => cpu.program_counter = nnn,
            2 => {
                cpu.stack.push(cpu.program_counter);
                cpu.program_counter = nnn;
            }
            3 => if cpu.v_registers[x as usize] as u16 == nn {cpu.program_counter += 2}
            4 => if cpu.v_registers[x as usize] as u16 != nn {cpu.program_counter += 2}
            5 => if cpu.v_registers[x as usize] == cpu.v_registers[y as usize] {cpu.program_counter += 2}
            6 => cpu.v_registers[x as usize] = nn as u8,
            7 => cpu.v_registers[x as usize] += nn as u8,
            8 => {
                match n {
                    0 => cpu.v_registers[x as usize] = cpu.v_registers[y as usize],
                    1 => cpu.v_registers[x as usize] = cpu.v_registers[x as usize] | cpu.v_registers[y as usize],
                    2 => cpu.v_registers[x as usize] = cpu.v_registers[x as usize] & cpu.v_registers[y as usize],
                    3 => cpu.v_registers[x as usize] = cpu.v_registers[x as usize] ^ cpu.v_registers[y as usize],
                    4 => {
                        cpu.v_registers[x as usize] += cpu.v_registers[y as usize];
                        if cpu.v_registers[x as usize] >= 255 {cpu.v_registers[0xF] = 1}
                        else {cpu.v_registers[0xF] = 0}
                    }
                    5 => {
                        if cpu.v_registers[x as usize] > cpu.v_registers[y as usize] {cpu.v_registers[0xF] = 1}
                        else if cpu.v_registers[x as usize] < cpu.v_registers[y as usize] {cpu.v_registers[0xF] = 0}
                        cpu.v_registers[x as usize] -= cpu.v_registers[y as usize];
                    }
                    6 => {
                        //cpu.v_registers[x as usize] = cpu.v_registers[y as usize]; COSMAC VIP
                        let s_bit = cpu.v_registers[x as usize] & 1;
                        cpu.v_registers[x as usize] = cpu.v_registers[x as usize] >> 1;
                        cpu.v_registers[0xF] = s_bit;
                    }
                    7 => {
                        if cpu.v_registers[x as usize] > cpu.v_registers[y as usize] {cpu.v_registers[0xF] = 1}
                        else if cpu.v_registers[x as usize] < cpu.v_registers[y as usize] {cpu.v_registers[0xF] = 0}
                        cpu.v_registers[x as usize] = cpu.v_registers[y as usize] - cpu.v_registers[x as usize];
                    }
                    0xE => {
                        //cpu.v_registers[x as usize] = cpu.v_registers[y as usize]; COSMAC VIP
                        let s_bit = (cpu.v_registers[x as usize] >> 7) & 1;
                        cpu.v_registers[x as usize] = cpu.v_registers[x as usize] << 1;
                        cpu.v_registers[0xF] = s_bit;
                    }
                    _ => {}
                }
            }
            9 => if cpu.v_registers[x as usize] != cpu.v_registers[y as usize] {cpu.program_counter += 2}
            0xA => cpu.index_register = nnn,
            0xB => {
                //cpu.program_counter = nnn + cpu.v_registers[0] as u16; COSMAC VIP
                cpu.program_counter = nnn + cpu.v_registers[x as usize] as u16;
            }
            0xC => {
                let mut rand = rand::thread_rng().gen_range(0..nn);
                cpu.v_registers[x as usize] = rand & nn;
            }
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
            0xE => {
                match y {
                    9 => {}
                    0xA => {}
                    _ => {}
                }
            }
            0xF => {}
            _ => {}
        }
        sleep(time::Duration::from_millis(100))
    }
    
}
