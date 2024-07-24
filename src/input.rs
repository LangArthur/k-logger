const EV_KEY: u16 = 1;

#[repr(C)]
pub struct Event {
    pub tv_sec: isize,
    pub tv_usec: isize,
    pub evt_type: u16,
    pub code: u16,
    pub value: i32,
}

impl Event {
    pub fn from_buffer(buff: &[u8; 24]) -> Self {
        unsafe { std::mem::transmute::<[u8; 24], Self>(*buff) }
    }

    pub fn is_key(&self) -> bool {
        self.evt_type == EV_KEY
    }

    pub fn is_pressed(&self) -> bool {
        self.value == 1
    }

    pub fn is_released(&self) -> bool {
        self.value == 0
    }
}
