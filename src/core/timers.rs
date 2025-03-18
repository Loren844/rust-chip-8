pub struct Timers {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Timers {
    pub fn new() -> Self {
        Timers {
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn update(&mut self) {
        if self.delay_timer > 0 {self.delay_timer -= 1};
        if self.sound_timer > 0 {self.sound_timer -= 1};
    }
}