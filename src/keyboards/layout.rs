use crate::key::{self, KeyError};

pub trait Layout {
    fn to_key(code: u16) -> Result<key::Code, KeyError>;
    fn shift(key: &key::Code) -> String;
}

pub const TO_CAPITALIZE: [key::Code; 26] = [
    key::Code::KEY_Q,
    key::Code::KEY_W,
    key::Code::KEY_E,
    key::Code::KEY_R,
    key::Code::KEY_T,
    key::Code::KEY_Y,
    key::Code::KEY_U,
    key::Code::KEY_I,
    key::Code::KEY_O,
    key::Code::KEY_P,
    key::Code::KEY_A,
    key::Code::KEY_S,
    key::Code::KEY_D,
    key::Code::KEY_F,
    key::Code::KEY_G,
    key::Code::KEY_H,
    key::Code::KEY_J,
    key::Code::KEY_K,
    key::Code::KEY_L,
    key::Code::KEY_Z,
    key::Code::KEY_X,
    key::Code::KEY_C,
    key::Code::KEY_V,
    key::Code::KEY_B,
    key::Code::KEY_N,
    key::Code::KEY_M,
];
