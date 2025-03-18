mod core;
mod display;

use crate::core::memory::Memory;
use crate::display::screen::Screen;

fn main() {
    println!("Hello, world!");
    print!("I will try to learn Rust. ");
    println!("For this, I want to build a CHIP-8 emulator.");
    println!("I hope I can do it !");
    println!("Project started on 04/03/2025");

    let _mem = Memory::new();
    let _dis = Screen::new();
}
