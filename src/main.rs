mod core;
mod display;
mod gui;

use core::cpu::Cpu;
use display::screen::Screen;

fn main() -> Result<(), String> {
    let mut cpu: Cpu = Cpu::new();
    let mut screen: Screen = Screen::new();
    loop {
        //fetch
        let instruction: u16 = cpu.read_instruction();

        //decode
        let first_nibble: u16 = instruction >> 12;
        let x = (instruction >> 8) & 0xF;
        let y = (instruction >> 4) & 0xF;
        let n = instruction & 0xF;
        let nn = (instruction & 0xFF) as u8;
        let nnn = instruction & 0xFFF;

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
            6 => cpu.v_registers[x as usize] = nn,
            7 => cpu.v_registers[x as usize] += nn,
            8 => {}
            9 => {}
            0xA => cpu.index_register = nnn,
            0xB => {}
            0xC => {}
            0xD => {}
            0xE => {}
            0xF => {}
            _ => {}
        }
    }
}
