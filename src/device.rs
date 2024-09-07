use std::{path::Path, str::FromStr};

use tracing::warn;

use crate::{keyboards::Variant, utils::get_file_content};

const DEVICES_PATH: &str = "/proc/bus/input/devices";
const INPUT_FILE: &str = "/dev/input/";

const KEYBOARD_MAPPING_FILES: [&str; 2] = ["/etc/default/keyboard", "/etc/rc.d/rc.keymap"];

pub struct InputDevice {
    pub name: String,
    pub events_supported: u32,
    pub events_fs: String,
}

impl InputDevice {
    fn from_bus_info(device_info: &str) -> std::io::Result<Self> {
        let mut name = String::new();
        let mut events_supported = u32::default();
        let mut events_fs = String::default();
        for line in device_info.split('\n') {
            if let Some(first_char) = line.chars().next() {
                match first_char {
                    'N' => name = line.strip_prefix("N: Name=").unwrap_or(line).to_string(),
                    'B' if line.contains("B: EV") => {
                        events_supported = line
                            .strip_prefix("B: EV=")
                            .expect("\"B: EV=\" should be present in bus information")
                            .parse()
                            .unwrap_or_else(|err| {
                                warn!("Failed to parse EV value for {name}: {err}. Use 0 instead");
                                0
                            });
                    }
                    'H' => {
                        events_fs = INPUT_FILE.to_owned()
                            + line
                                .strip_prefix("H: Handlers=")
                                .expect("Handlers should be present in bus information")
                                .split(' ')
                                .find(|handler| handler.contains("event"))
                                .expect("An event handler should be defined in bus information")
                    }
                    _ => (),
                }
            }
        }
        Ok(InputDevice {
            name,
            events_supported,
            events_fs,
        })
    }

    fn is_keyboard(&self) -> bool {
        self.events_supported & 120013 == 120013
    }
}

pub fn load_devices() -> std::io::Result<Vec<InputDevice>> {
    Ok(get_file_content(DEVICES_PATH)
        .unwrap_or_else(|_| panic!("{DEVICES_PATH} should be utf8 convertible"))
        .split("\n\n")
        .filter_map(|info| InputDevice::from_bus_info(info).ok())
        .collect())
}

pub fn detect_keyboard() -> Vec<InputDevice> {
    let mut res = load_devices().unwrap_or_default();
    res.retain(|d| d.is_keyboard());
    res
}

pub fn keyboard_layout() -> Variant {
    let path = match KEYBOARD_MAPPING_FILES
        .iter()
        .find(|path| Path::new(path).is_file())
    {
        Some(path) => Path::new(path),
        None => {
            warn!("Failed to deduce keyboard layout, using qwerty by default. Reason: cannot find layout file");
            return Variant::Qwerty;
        }
    };
    match get_file_content(path) {
        Ok(content) => {
            let lines: Vec<&str> = content.split_terminator('\n').collect();
            Variant::from_str(lines[1].strip_prefix("XKBVARIANT=").unwrap_or(lines[1]))
                .unwrap_or_else(|err| {
                    warn!(
                        "Failed to deduce keyboard layout, using qwerty by default. Reason: ${err}"
                    );
                    Variant::Qwerty
                })
        }
        Err(err) => {
            warn!("Failed to deduce keyboard layout, using qwerty by default. Reason: ${err}");
            Variant::Qwerty
        }
    }
}
