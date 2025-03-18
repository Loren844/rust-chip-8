# rust-chip-8
A Chip-8 emulator in rust

## Specificities 
- 🚫 The `0NNN` instruction is not implemented because it executes machine code specific to the original CHIP-8 processors (COSMAC VIP, ETI-660), which is irrelevant for a modern emulator.

## Missing features
- For the moment, the original COSMAC VIP mode is not available but it will be in a next release !

## Resources
### Rust Documentation
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [SDL2 Rust Documentation](https://docs.rs/sdl2/latest/sdl2/)
- [Rust SDL2 crates.io](https://crates.io/crates/sdl2)

### Chip-8 Documentation
- [Tobias V. Langhoff "Guide to making a CHIP-8 emulator"](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Wikipedia: CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)

### Chip-8 Games
- [CHIP-8 Archive (Games)](https://johnearnest.github.io/chip8Archive/)