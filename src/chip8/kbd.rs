pub struct Keyboard {
    keys: [bool; 16],
    kp: Option<u8>,
    kp_wait: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
            kp: None,
            kp_wait: false,
        }
    }

    pub fn set_keys(&mut self, keys: [bool; 16]) {
        self.keys = keys;
    }

    pub fn get_key(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn push_keypress(&mut self, key: u8) {
        if self.kp_wait {
            self.kp = Some(key);
        }
    }

    pub fn wait_keypress(&mut self) -> Option<u8> {
        if self.kp_wait {
            match self.kp {
                Some(key) => {
                    self.kp = None;
                    self.kp_wait = false;
                    Some(key)
                },
                None => None
            }
        } else {
            self.kp = None;
            self.kp_wait = true;
            None
        }
    }
}