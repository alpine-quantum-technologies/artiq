use crate::i2c;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::Deserialize_repr;

mod config;
mod status;

pub use config::{Config, InvalidConfig};
pub use status::{ChannelFlags, IfcMode, Status};

/// Reference clock input selection.
///
/// Only supports Urukul v1.3+.
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Deserialize_repr)]
#[repr(u8)]
pub enum ClkSel {
    /// On-board crystal, 100 MHz
    Internal = 0,

    /// Front-panel SMA connector.
    Sma = 1,

    /// Internal MMCX connector.
    Mmcx = 2,
}

impl Default for ClkSel {
    fn default() -> Self {
        Self::Internal
    }
}

/// Reference clock input divider.
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Deserialize_repr)]
#[repr(u8)]
pub enum ClkDiv {
    Default = 0,

    /// Divide by one.
    One = 1,

    /// Divide by two.
    Two = 2,

    /// Divide by four.
    Four = 3,
}

impl Default for ClkDiv {
    fn default() -> Self {
        Self::Default
    }
}

impl ClkDiv {
    /// Reference clock input divider, as integer.
    pub fn divider(&self) -> i32 {
        match self {
            Self::Default | Self::Four => 4,
            Self::One => 1,
            Self::Two => 2,
        }
    }
}

/// DDS inter-chip synchronization clock source selection.
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Deserialize_repr)]
#[repr(u8)]
pub enum SyncSel {
    /// EEM1 connector, LVDS pair 0.
    Eem = 0,

    /// On-board DDS chip 0.
    Dds0 = 1,
}

impl Default for SyncSel {
    fn default() -> Self {
        Self::Dds0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SyncDataSource {
    Eeprom {
        port: i2c::KasliPort,
        offset: u32,
    },
    User {
        sync_delay_seed: i32,
        io_update_delay: i32,
    },
}
