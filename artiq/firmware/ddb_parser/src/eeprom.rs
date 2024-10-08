use serde::Deserialize;
use serde_with::{serde_as, TryFromInto};
use std::fmt;

#[serde_as]
#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub struct Kasli {
    #[serde_as(as = "TryFromInto<&str>")]
    pub port: sinara_config::i2c::KasliPort,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum MaybeOnEeprom {
    EepromAddress(EepromAddress),
    Value(i32),
}

impl Default for MaybeOnEeprom {
    fn default() -> Self {
        Self::Value(0)
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(try_from = "&str")]
pub struct EepromAddress {
    pub eeprom_device: String,
    pub offset: u32,
}

impl TryFrom<&str> for EepromAddress {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(':');

        let eeprom_device = parts
            .next()
            .ok_or_else(|| Self::Error::InvalidDesignator(value.into()))?
            .into();
        let offset = parts
            .next()
            .ok_or_else(|| Self::Error::InvalidDesignator(value.into()))?
            .parse::<_>()
            .map_err(|_| Self::Error::InvalidOffset)?;

        Ok(Self {
            eeprom_device,
            offset,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    InvalidDesignator(String),
    InvalidOffset,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDesignator(value) => write!(f, "Invalid EEPROM designator: {}", value),
            Self::InvalidOffset => write!(f, "Invalid EEPROM offset"),
        }
    }
}
