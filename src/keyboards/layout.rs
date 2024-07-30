use crate::key;

pub trait Layout {
    fn format(key: &key::Code, is_shifted: bool) -> String;
}
