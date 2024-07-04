use core::convert::{TryFrom, TryInto};

use super::ttl;
use crate::spi2;
use sinara_config::urukul::InvalidConfig;

mod ad9910;
mod cpld;

pub use self::cpld::{ChannelDesc, Cpld};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    Spi(spi2::Error),
    Config(InvalidConfig),
    ProtoRevMismatch,
    Ad9910AuxDacMismatch,
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

/// Synchronization generator, when using the EEM variant.
#[derive(Debug)]
pub struct SyncGen {
    /// Clock generator.
    pub device: ttl::TtlClockGen,

    /// Synchronization clock divider.
    pub div: u8,
}

/// SPI bus configuration.
pub(crate) struct SpiConfig {}

impl SpiConfig {
    // SPI clock dividers for configuration register read/write.
    const DIV_CFG_WR: i32 = 2;
    const DIV_CFG_RD: i32 = 16;

    // SPI clock dividers for coarse attenuation read/write.
    const DIV_ATT_WR: i32 = 6;
    const DIV_ATT_RD: i32 = 16;

    // SPI clock dividers for DDS registers read/write.
    const DIV_DDS_WR: i32 = 2;
    const DIV_DDS_RD: i32 = 16;

    const FLAGS: spi2::Flags = spi2::Flags::CsPolarity;
}

/// SPI target selection.
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum Cs {
    /// Configuration register.
    Cfg = 1,

    /// Coarse attenuators.
    Att = 2,

    /// Multiple DDS chip, as selected by `mask_nu` in the configuration register.
    DdsMulti = 3,

    /// DDS chip 0.
    DdsCh0 = 4,

    /// DDS chip 1.
    DdsCh1 = 5,

    /// DDS chip 2.
    DdsCh2 = 6,

    /// DDS chip 3.
    DdsCh3 = 7,
}

/// Urukul board channel.
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Channel {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl From<Channel> for usize {
    fn from(channel: Channel) -> Self {
        let index: u8 = channel.into();
        index.into()
    }
}

impl TryFrom<usize> for Channel {
    type Error = <Channel as TryFrom<u8>>::Error;

    fn try_from(value: usize) -> core::result::Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}

impl From<Channel> for Cs {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Zero => Self::DdsCh0,
            Channel::One => Self::DdsCh1,
            Channel::Two => Self::DdsCh2,
            Channel::Three => Self::DdsCh3,
        }
    }
}
