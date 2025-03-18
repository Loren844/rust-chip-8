pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Screen {
    pub pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    pub fn new() -> Self {
        let pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
        Screen { pixels }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }

    pub fn swap(&mut self, x: usize, y: usize) {
        self.pixels[y][x] = !self.pixels[y][x];
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    }

    pub fn get_width(&self) -> u8 {
        SCREEN_WIDTH as u8
    }

    pub fn get_height(&self) -> u8 {
        SCREEN_HEIGHT as u8
    }

    pub fn draw(&self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                match self.get_pixel(x, y) {
                    true => print!("XX"),
                    false => print!(" "),
                }
                if x == SCREEN_WIDTH - 1 {
                    println!();
                }
            }
        }
    }
}
