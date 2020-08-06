use super::super::platform::Platform;

const DISPLAY_W: usize = 64;
const DISPLAY_H: usize = 32;

pub struct Display {
    pixels: [bool; DISPLAY_W * DISPLAY_H],
    redraw: bool,
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [false; DISPLAY_W * DISPLAY_H],
            redraw: true,
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [false; DISPLAY_W * DISPLAY_H];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: bool) -> bool {
        let x = x % DISPLAY_W;
        let y = y % DISPLAY_H;
        let addr = x + (y * DISPLAY_W);
        if self.pixels[addr] != pixel {
            self.redraw = true;
        }
        let erased = self.pixels[addr] && pixel; // pixel changed from 1 to 0
        self.pixels[addr] ^= pixel;
        erased
    }

    pub fn height(&self) -> usize {
        DISPLAY_H
    }

    #[allow(unused)]
    pub fn print(&self) {
        for y in 0..DISPLAY_H {
            for x in 0..DISPLAY_W {
                let addr = x + (y * DISPLAY_W);
                if self.pixels[addr] {
                    print!("\u{2588}");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn draw(&mut self, platform: &mut Platform) {
        for y in 0..DISPLAY_H {
            for x in 0..DISPLAY_W {
                let addr = x + (y * DISPLAY_W);
                if self.pixels[addr] {
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