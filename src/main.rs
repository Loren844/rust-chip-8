mod core;
mod gui;
mod utils;

use core::chip8::Chip8;
use gui::window::Gui;
use utils::loader::load_rom;

use sdl2::event::Event;
use std::io;
use std::time::{Duration, Instant};
use std::{thread::sleep, time};

fn main() -> Result<(), String> {
    let mut chip8: Chip8 = Chip8::new();
    let mut path = String::new();
    //selection de la ROM
    while path.trim().is_empty() {
        println!("Saisissez le chemin d'accès vers la ROM :");
        io::stdin()
            .read_line(&mut path)
            .expect("Échec de la lecture de l'entrée");
        path = path.trim().to_string();

        match load_rom(&path) {
            Ok(rom) => chip8.memory.load_rom(rom),
            Err(_) => {
                eprintln!("Une erreur est survenue en chargeant la ROM !");
                path = String::new();
            }
        }
    }

    let mut last_timer_update = Instant::now();

    let sdl_context = sdl2::init()?;
    let mut gui = Gui::new(&sdl_context)?;
    let mut event_pump = sdl_context.event_pump()?;

    //boucle infinie
    loop {
        //fetch
        let instruction: u16 = chip8.read_instruction();

        //decode and execute
        chip8.decode_and_execute(instruction, &event_pump);

        //show screen
        gui.draw_screen(&chip8.screen);

        //handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    gui.quit();
                    break;
                }
                _ => {}
            }
        }
        if last_timer_update.elapsed() >= Duration::from_millis(16) {
            chip8.timers.update();
            last_timer_update = Instant::now();
        }
        sleep(time::Duration::from_millis(16))
    }
}
