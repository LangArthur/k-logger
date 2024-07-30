use std::marker::PhantomData;

use crate::{input, key::Code, keyboards::Layout};

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

pub struct Console<T>
where
    T: Layout,
{
    shift_pressed: u8,
    phantom: std::marker::PhantomData<T>,
    // session: Session<F>,
}

impl<T> Console<T>
where
    T: Layout,
{
    pub fn new() -> Self {
        Self {
            shift_pressed: 0,
            phantom: PhantomData,
            // session: Session::new(|| println!("\n")),
        }
    }

    fn format(&self, key: &Code) -> String {
        if self.shift_pressed > 0 {
            <T>::shift(key)
        } else {
            key.to_string()
        }
    }
}

impl<T> Consumer for Console<T>
where
    T: Layout,
{
    fn consume(&mut self, event: input::Event) {
        // self.session.update();
        if event.is_key() {
            let key: Code = <T>::to_key(event.code).unwrap();
            if event.is_pressed() {
                if key == Code::KEY_LEFTSHIFT || key == Code::KEY_RIGHTSHIFT {
                    self.shift_pressed += 1
                }
                println!("{}", Console::<T>::format(self, &key));
            } else if event.is_released()
                && (key == Code::KEY_LEFTSHIFT || key == Code::KEY_RIGHTSHIFT)
            {
                self.shift_pressed -= 1
            }
        }
    }
}
