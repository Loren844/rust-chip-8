use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::core::screen::Screen;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const PIXEL_SIZE: u32 = 10;
const OFF_COLOR: Color = Color::RGB(0, 0, 0);
const ON_COLOR: Color = Color::RGB(255, 255, 255);

pub struct Gui {
    canvas: Canvas<Window>,
}

impl Gui {
    pub fn new(sdl_context: &Sdl) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(
                "CHIP-8 Emulator",
                SCREEN_WIDTH * PIXEL_SIZE,
                SCREEN_HEIGHT * PIXEL_SIZE,
            )
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Gui { canvas })
    }

    pub fn draw_screen(&mut self, screen: &Screen) {
        self.canvas.set_draw_color(OFF_COLOR);
        self.canvas.clear();

        self.canvas.set_draw_color(ON_COLOR);
        for (y, row) in screen.pixels.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                if pixel {
                    let rect = Rect::new(
                        (x as u32 * PIXEL_SIZE) as i32,
                        (y as u32 * PIXEL_SIZE) as i32,
                        PIXEL_SIZE,
                        PIXEL_SIZE,
                    );
                    self.canvas.fill_rect(rect).unwrap();
                }
            }
        }

        self.canvas.present();
    }

    pub fn quit(&mut self) {
        std::process::exit(0)
    }
}
