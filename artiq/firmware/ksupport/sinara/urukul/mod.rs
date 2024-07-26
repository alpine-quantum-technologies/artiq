use core::convert::{TryFrom, TryInto};

use super::ttl;
use crate::{nrt_bus::i2c, spi2};
use sinara_config::urukul::{InvalidConfig, SyncDataSource};

mod ad9910;
mod cpld;

pub use self::cpld::{ChannelDesc, Cpld};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    Spi(spi2::Error),
    I2c(i2c::Error),
    Config(InvalidConfig),
    ProtoRevMismatch,
    Ad9910AuxDacMismatch,
    PllNotLocked(Channel),
}

impl From<spi2::Error> for Error {
    fn from(other: spi2::Error) -> Self {
        Self::Spi(other)
    }
}

impl From<i2c::Error> for Error {
    fn from(other: i2c::Error) -> Self {
        Self::I2c(other)
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

/// Synchronization data.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct SyncData {
    pub sync_delay_seed: u8,
    pub io_update_delay: u8,
    pub validation_window: u8,
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

impl From<Channel> for sinara_config::urukul::ChannelFlags {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Zero => Self::Zero,
            Channel::One => Self::One,
            Channel::Two => Self::Two,
            Channel::Three => Self::Three,
        }
    }
}

impl SyncData {
    pub const fn new() -> Self {
        Self {
            sync_delay_seed: 0,
            io_update_delay: 0,
            validation_window: 0,
        }
    }

    pub fn init(source: SyncDataSource, i2c_bus: &i2c::KasliI2C) -> Result<Self> {
        match source {
            SyncDataSource::User {
                sync_delay_seed,
                io_update_delay,
                validation_window,
            } => Ok(Self {
                sync_delay_seed,
                io_update_delay,
                validation_window,
            }),
            SyncDataSource::Eeprom { port, offset } => {
                let word = i2c_bus.eeprom_read_i32(port, offset as i32)?;
                // EEPROM layout:
                // | sync_delay_seed (8) | io_update_delay (8) | validation_window (8) | reserved (8) |
                let word = word >> 8;
                let validation_window = (word & 0xff) as u8;
                let word = word >> 8;
                let sync_delay_seed = ((word >> 8) & 0xff) as u8;
                let io_update_delay = if word & 0xff != 0xff {
                    (word & 0xff) as u8
                } else {
                    0
                };

                Ok(Self {
                    sync_delay_seed,
                    io_update_delay,
                    validation_window,
                })
            }
        }
    }
}

impl Default for SyncData {
    fn default() -> Self {
        Self::new()
    }
}
