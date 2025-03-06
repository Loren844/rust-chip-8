mod gui;
mod display;

use display::screen::Screen;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use gui::window::Gui;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut gui = Gui::new(&sdl_context)?;

    let mut screen:Screen = Screen::new();

    let mut event_pump = sdl_context.event_pump()?;
    let mut cpt_x:usize = 0;
    let mut cpt_y:usize = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        screen.swap(cpt_x, cpt_y);
        cpt_x+=1;

        if cpt_x > 63 {
            cpt_x = 0;
            cpt_y+=1;
        };
        if cpt_y > 31 {cpt_y = 0;}
    

        gui.draw_screen(&screen.pixels);
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}