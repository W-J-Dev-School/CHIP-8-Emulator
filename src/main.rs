use chip8::CHIP8;

mod chip8;
mod platform;

fn main() {
    let mut chip8 = CHIP8::new();

    match chip8.load_rom("roms/TANK") {
        Ok(()) => (),
        Err(e) => panic!("Failed to load ROM: {}", e),
    }

    chip8.run();
}
