use crate::spi2;
use sinara_config::urukul::InvalidConfig;

mod cpld;

pub use self::cpld::Cpld;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    Spi(spi2::Error),
    Config(InvalidConfig),
    ProtoRevMismatch,
}

impl From<spi2::Error> for Error {
    fn from(other: spi2::Error) -> Self {
        Self::Spi(other)
    }
}

impl From<InvalidConfig> for Error {
    fn from(other: InvalidConfig) -> Self {
        Self::Config(other)
    }
}
