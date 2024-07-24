use crate::{input, keys::Code};

pub trait Consumer {
    fn consume(&mut self, event: input::Event);
}

pub struct Console {
    shift_pressed: u8,
}

impl Console {
    const TO_CAPITALIZE: [Code; 26] = [
        Code::KEY_Q,
        Code::KEY_W,
        Code::KEY_E,
        Code::KEY_R,
        Code::KEY_T,
        Code::KEY_Y,
        Code::KEY_U,
        Code::KEY_I,
        Code::KEY_O,
        Code::KEY_P,
        Code::KEY_A,
        Code::KEY_S,
        Code::KEY_D,
        Code::KEY_F,
        Code::KEY_G,
        Code::KEY_H,
        Code::KEY_J,
        Code::KEY_K,
        Code::KEY_L,
        Code::KEY_Z,
        Code::KEY_X,
        Code::KEY_C,
        Code::KEY_V,
        Code::KEY_B,
        Code::KEY_N,
        Code::KEY_M,
    ];

    pub fn new() -> Self {
        Self { shift_pressed: 0 }
    }

    fn format(&self, key: &Code) -> String {
        let to_print = key.to_string();
        if self.shift_pressed > 0 {
            if Self::TO_CAPITALIZE.contains(key) {
                return to_print.to_uppercase();
            }
        }
        to_print
    }
}

impl Consumer for Console {
    fn consume(&mut self, event: input::Event) {
        if event.is_key() {
            let key: Code = event.code.try_into().unwrap();
            if event.is_pressed() {
                if key == Code::KEY_LEFTSHIFT || key == Code::KEY_RIGHTSHIFT {
                    self.shift_pressed += 1
                }
                println!("{}", self.format(&key));
            } else if event.is_released() {
                if key == Code::KEY_LEFTSHIFT || key == Code::KEY_RIGHTSHIFT {
                    self.shift_pressed -= 1
                }
            }
        }
    }
}
