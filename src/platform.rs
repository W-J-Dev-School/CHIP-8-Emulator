use fermium::*;
use std::ffi::CString;
use std::ptr::null_mut;

struct Keymap;

impl Keymap {
    pub fn new() -> Keymap {
        Self
    }

    pub fn keycode(&self, key: u8) -> i32 {
        /*
            CHIP-8 layout is:
            1	2	3	C
            4	5	6	D
            7	8	9	E
            A	0	B	F
        */
        match key {
            0x1 => SDLK_1,
            0x2 => SDLK_2,
            0x3 => SDLK_3,
            0xC => SDLK_4,
            0x4 => SDLK_q,
            0x5 => SDLK_w,
            0x6 => SDLK_e,
            0xD => SDLK_r,
            0x7 => SDLK_a,
            0x8 => SDLK_s,
            0x9 => SDLK_d,
            0xE => SDLK_f,
            0xA => SDLK_z,
            0x0 => SDLK_x,
            0xB => SDLK_c,
            0xF => SDLK_v,
            _ => panic!("Key out of range"),
        }
    }
}

const DISPLAY_W: i32 = 64;
const DISPLAY_H: i32 = 32;
const DISPLAY_SCALE: i32 = 20;

pub enum PlatformEvent {
    KeyPress(u8),
    Quit,
    None,
}

pub struct Platform {
    #[allow(unused)]
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    keymap: Keymap,
}

impl Platform {
    pub fn new() -> Self {
        unsafe {
            if SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_EVENTS | SDL_INIT_TIMER) != 0 {
                panic!("Can't init SDL");
            }

            let name = CString::new("CHIP-8").unwrap();
            let w = DISPLAY_W * DISPLAY_SCALE;
            let h = DISPLAY_H * DISPLAY_SCALE;
            let flags = (SDL_WINDOW_SHOWN | SDL_WINDOW_ALLOW_HIGHDPI) as u32;

            let window = SDL_CreateWindow(name.as_ptr(), SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, w, h,
                flags);
            if window.is_null() {
                panic!("Can't create window");
            }

            let flags = (SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC) as u32;

            let renderer = SDL_CreateRenderer(window, -1, flags);
            if renderer.is_null() {
                panic!("Can't create renderer");
            }

            let keymap = Keymap::new();

            let mut audio_spec_want = SDL_AudioSpec {
                freq: 523, // C5
                format: AUDIO_S16 as u16,
                channels: 1,
                samples: 32,
                callback: Some(audio_callback),
                ..SDL_AudioSpec::default()
            };
            if SDL_OpenAudio(&mut audio_spec_want, null_mut()) != 0 {
                panic!("Can't create audio device");
            }

            Self { window, renderer, keymap }
        }
    }

    pub fn keyboard_state(&mut self) -> [bool; 16] {
        unsafe {
            let state = SDL_GetKeyboardState(null_mut());
            let state = std::slice::from_raw_parts(state, SDL_NUM_SCANCODES as usize);
            let mut keys = [false; 16];
            for key in 0..0xF {
                if state[SDL_GetScancodeFromKey(self.keymap.keycode(key)) as usize] == 1 {
                    keys[key as usize] = true;
                }
            }
            keys
        }
    }

    pub fn beep(&mut self, beep: bool) {
        unsafe {
            SDL_PauseAudio(if beep { 0 } else { 1 });
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, 0x19, 0x14, 0x28, 0xFF);
            SDL_RenderClear(self.renderer);
        }
    }

    pub fn present(&mut self) {
        unsafe {
            SDL_RenderPresent(self.renderer);
        }
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8) {
        unsafe {
            let rect = SDL_Rect {
                x: DISPLAY_SCALE as i32 * x as i32,
                y: DISPLAY_SCALE as i32 * y as i32,
                w: DISPLAY_SCALE as i32 * 1,
                h: DISPLAY_SCALE as i32 * 1,
            };
            SDL_SetRenderDrawColor(self.renderer, 0xC8, 0xC8, 0xFF, 0xFF);
            SDL_RenderFillRect(self.renderer, &rect);
        }
    }

    pub fn poll_event(&mut self) -> PlatformEvent {
        unsafe {
            let mut event = SDL_Event::default();
            SDL_PollEvent(&mut event);
            match event.type_ as i32 {
                SDL_KEYDOWN => {
                    let mut result = PlatformEvent::None;
                    for key in 0..0xF {
                        if event.key.keysym.sym == self.keymap.keycode(key) {
                            result = PlatformEvent::KeyPress(key);
                            break;
                        }
                    }
                    result
                },
                SDL_QUIT => PlatformEvent::Quit,
                _ => PlatformEvent::None
            }
        }
    }
}

unsafe extern "C" fn audio_callback(_userdata: *mut c_void, stream: *mut Uint8, len: c_int) {
    let stream = std::slice::from_raw_parts_mut(stream as *mut i16, len as usize);
    for i in 0..len {
        if i % 2 == 0 {
            stream[i as usize] = i16::MAX;
        } else {
            stream[i as usize] = i16::MIN;
        }
    }
}