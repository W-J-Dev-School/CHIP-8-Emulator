use super::super::platform::Platform;
use std::fmt;

const DISPLAY_W: usize = 64;
const DISPLAY_H: usize = 32;

pub struct Display {
    pixels: [[bool; DISPLAY_H]; DISPLAY_W],
    redraw: bool,
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [[false; DISPLAY_H]; DISPLAY_W],
            redraw: true,
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; DISPLAY_H]; DISPLAY_W];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: bool) -> bool {
        let x = x % DISPLAY_W;
        let y = y % DISPLAY_H;
        if self.pixels[x][y] != pixel {
            self.redraw = true;
        }
        let erased = self.pixels[x][y] && pixel; // pixel changed from 1 to 0
        self.pixels[x][y] ^= pixel;
        erased
    }

    pub fn height(&self) -> usize {
        DISPLAY_H
    }

    pub fn draw(&mut self, platform: &mut Platform) {
        for y in 0..DISPLAY_H {
            for x in 0..DISPLAY_W {
                if self.pixels[x][y] {
                    platform.draw_pixel(x as u8, y as u8);
                }
            }
        }
        self.redraw = false;
    }

    pub fn redraw(&self) -> bool {
        self.redraw
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..DISPLAY_H {
            for x in 0..DISPLAY_W {
                if self.pixels[x][y] {
                    write!(f, "\u{2588}")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}