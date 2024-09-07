use std::io::Read;

use consumer::Consumer;
use device::{keyboard_layout, InputDevice};
use tracing::info;

mod consumer;
mod device;
mod input;
mod key;
mod keyboards;
mod utils;

fn select_keyboard(keyboards: &[InputDevice]) -> &InputDevice {
    if keyboards.len() > 1 {
        println!("Several keyboards has been detected, please select one: ");
        let mut idx = 1;
        for keyboard in keyboards {
            println!("[{}] - {}", idx, keyboard.name);
            idx += 1;
        }
        let mut attempt = 0;
        let mut buffer = String::new();
        while attempt < 2 {
            std::io::stdin().read_line(&mut buffer).unwrap();
            if let Ok(value) = buffer.trim().parse::<usize>() {
                let value = value - 1;
                if value < keyboards.len() {
                    return &keyboards[value];
                }
            }
            attempt += 1;
        }
        println!("Failed to select a valid keyboard, taking the first one instead");
    }
    &keyboards[0]
}

fn is_running() -> bool {
    true
}

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let keyboards = device::detect_keyboard();
    let to_listen = select_keyboard(&keyboards);
    info!("Listen inputs from {}", to_listen.name);
    println!("{:?}", to_listen.events_fs);
    let mut buffer = [0u8; 24];
    let variant = keyboard_layout();
    info!("Detected a {variant} keyboard (not fully implemented yet");

    let mut console = consumer::Console::<keyboards::Azerty>::new();
    // let mut console: Box<consumer::Console<dyn KeyMap>> = match variant {
    //     Variant::Azerty => Box::new(consumer::Console::<keyboards::Azerty>::new()),
    //     Variant::Qwerty => Box::new(consumer::Console::<keyboards::Qwerty>::new()),
    // };

    let mut fd = std::fs::File::open(to_listen.events_fs.clone())?;
    while is_running() {
        let n = fd.read(&mut buffer)?;
        if n > 0 {
            let event = input::Event::from_buffer(&buffer);
            if event.is_key() {
                console.consume(event);
            }
        }
    }
    Ok(())
}
