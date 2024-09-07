use std::{fmt::Display, str::FromStr};

use crate::key;

// this should be extend in a near future
pub enum VariantError {
    UnknownVariant(String),
}

impl Display for VariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariantError::UnknownVariant(variant) => write!(f, "Unknown variant {variant}"),
        }
    }
}

pub enum Variant {
    Azerty,
    Qwerty,
}

impl FromStr for Variant {
    type Err = VariantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "azerty" => Ok(Self::Azerty),
            "qwerty" => Ok(Self::Qwerty),
            variant => Err(VariantError::UnknownVariant(variant.to_string())),
        }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Azerty => write!(f, "azerty"),
            Variant::Qwerty => write!(f, "qwerty"),
        }
    }
}

pub trait KeyMap {
    fn format(key: &key::Code, is_shifted: bool) -> String;
}
