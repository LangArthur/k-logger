use crate::{input, key::Code, keyboards::KeyMap};

pub trait Consumer {
    fn consume(&mut self, event: input::Event);
}

/// FIXME: this is not used at the moment
// struct Session<F: Fn()> {
//     stop: Option<std::time::SystemTime>,
//     callable: F,
// }

// impl<F> Session<F>
// where
//     F: Fn(),
// {
//     pub fn new(callable: F) -> Self {
//         Self {
//             stop: None,
//             callable: callable,
//         }
//     }

//     pub fn update(&mut self) {
//         self.stop = Some(std::time::SystemTime::now() + std::time::Duration::from_millis(500));
//     }

//     fn run(&mut self) {}
// }

pub struct Console
{
    shift_pressed: u8,
    left_alt_pressed: bool,
    formatter: Box<dyn KeyMap>,
    // session: Session<F>,
}

impl Console
{
    pub fn new(formatter: Box<dyn KeyMap>) -> Self {
        Self {
            shift_pressed: 0,
            left_alt_pressed: false,
            formatter
            // session: Session::new(|| println!("\n")),
        }
    }
}

impl Consumer for Console
{
    fn consume(&mut self, event: input::Event) {
        // self.session.update();
        if event.is_key() {
            let key: Code = event.code.try_into().unwrap();
            if event.is_pressed() {
                if key == Code::KEY_LEFTSHIFT || key == Code::KEY_RIGHTSHIFT {
                    self.shift_pressed += 1;
                } else if key == Code::KEY_LEFTALT {
                    self.left_alt_pressed = true;
                }
                println!("{}", self.formatter.format(&key, self.shift_pressed > 0));
            } else if event.is_released() {
                if key == Code::KEY_LEFTSHIFT || key == Code::KEY_RIGHTSHIFT {
                    self.shift_pressed -= 1
                } else if key == Code::KEY_LEFTALT {
                    self.left_alt_pressed = false;
                }
            }
        }
    }
}
