use chip8::CHIP8;

mod chip8;
mod platform;

fn main() {
    let mut chip8 = CHIP8::new();
    chip8.load_rom("roms/BLINKY").expect("Failed to load ROM!");
    chip8.run();
}
