pub struct Screen {
    pub pixels:[[bool; 64]; 32],
}

impl Screen {
    pub fn new() -> Self {

        let pixels:[[bool;64];32] = [[false;64];32];
        Screen {
            pixels,
        }
    }

    pub fn swap(&mut self, x:usize, y:usize) {
        self.pixels[y][x] = !self.pixels[y][x];
    }
}