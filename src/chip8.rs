use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use clk::Clock;
use cpu::CPU;
use dsp::Display;
use kbd::Keyboard;
use mem::Memory;
use super::platform::{Platform, PlatformEvent};

mod clk;
mod cpu;
mod dsp;
mod kbd;
mod mem;
mod op;

pub struct CHIP8 {
    platform: Platform,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
    cpu: CPU,
}

impl CHIP8 {
    pub fn new() -> Self {
        Self {
            platform: Platform::new(),
            memory: Memory::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            cpu: CPU::new()
        }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let rom_file = File::open(path)?;
        let rom_file = BufReader::new(rom_file);

        let mut rom_size = 0;
        for byte in rom_file.bytes() {
            let addr = (0x200 + rom_size) as u16;
            rom_size += 1;
            self.memory.write(addr, byte?);
        }

        // println!("ROM size: {}", rom_size);
        // println!("ROM data:");

        // for i in 0..rom_size {
        //     let addr = (0x200 + i) as u16;
        //     let byte = self.memory.read(addr);
        //     print!("{:02x} ", byte);
        //     if (i + 1) % 16 == 0 {
        //         println!("")
        //     } else if (i + 1) % 4 == 0 {
        //         print!(" ");
        //     }
        // }

        Ok(())
    }

    pub fn run(&mut self) {
        let mut clkc = Clock::new(3000); // CPU clock
        let mut done = false;
        while !done {
            match self.platform.poll_event() {
                PlatformEvent::KeyPress(key) => {
                    self.keyboard.push_keypress(key);
                }
                PlatformEvent::Quit => {
                    done = true;
                }
                PlatformEvent::None => {
                    self.keyboard.set_keys(self.platform.keyboard_state());

                    while clkc.tick() {
                        self.cpu.cycle(&mut self.memory, &mut self.display, &mut self.keyboard);

                        // TODO: Don't call this every cycle
                        self.platform.beep(self.cpu.beep());
                    }

                    if self.display.redraw() {
                        self.platform.clear();
                        self.display.draw(&mut self.platform);
                    }

                    self.platform.present();
                }
            }
        }
    }
}
