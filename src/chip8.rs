use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use clk::Clock;
use cpu::CPU;
use dsp::Display;
use kbd::Keyboard;
use mem::Memory;
use rng::RNG;
use stt::Settings;
use super::platform::{Platform, PlatformEvent};

mod clk;
mod cpu;
mod dsp;
mod kbd;
mod mem;
mod op;
mod rng;
mod stt;

pub struct CHIP8 {
    settings: Settings,
    platform: Platform,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
    cpu: CPU,
}

impl CHIP8 {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
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

        if self.settings.print_rom {
            println!("ROM size: {}", rom_size);
            println!("ROM data:");

            for i in 0..rom_size {
                let addr = (0x200 + i) as u16;
                let byte = self.memory.read(addr);
                print!("{:02x} ", byte);
                if (i + 1) % 16 == 0 {
                    println!("")
                } else if (i + 1) % 4 == 0 {
                    print!(" ");
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self) {
        let mut cpu_clock = Clock::new(self.settings.cpu_freq);
        let mut dt_clock = Clock::new(self.settings.dt_freq);
        let mut st_clock = Clock::new(self.settings.st_freq);

        let mut rng = RNG::new(self.settings.rng_seed);

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
                    if st_clock.tick() {
                        let beep = self.cpu.cycle_st();
                        self.platform.beep(beep && !self.settings.mute);
                    }

                    if dt_clock.tick() {
                        self.cpu.cycle_dt();
                    }

                    if cpu_clock.tick() {
                        self.keyboard.set_keys(self.platform.keyboard_state());

                        self.cpu.cycle(
                            &mut self.memory,
                            &mut self.display,
                            &mut self.keyboard,
                            &mut rng,
                            &self.settings
                        );

                        if self.display.redraw() {
                            self.platform.clear();
                            self.display.draw(&mut self.platform);
                            self.platform.present();
                        }
                    }
                }
            }
        }
    }
}
