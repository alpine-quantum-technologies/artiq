mod config;
mod cpld;
mod status;

pub use self::config::{ClkDiv, ClkSel, Config, SyncSel};
pub use self::cpld::Cpld;
pub use self::status::{IfcMode, Status};

use crate::spi2;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    Spi(spi2::Error),
    InvalidClkSel(i32),
    InvalidSyncSel(i32),
    InvalidClkDiv(i32),
    ProtoRevMismatch,
}

impl From<spi2::Error> for Error {
    fn from(other: spi2::Error) -> Self {
        Self::Spi(other)
    }
}
