use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::Deserialize_repr;

mod config;
mod registers;
mod spi_config;
mod status;

mod dac34h84;

pub use config::Config;
pub use dac34h84::DAC_MMAP;
pub use registers::{Address, BOARD_ID};
pub use spi_config::{SpiConfig, SpiCs};
pub use status::Status;

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Deserialize_repr)]
#[repr(u8)]
pub enum ClkSel {
    Mmcx = 0,
    Sma = 1,
}

impl Default for ClkSel {
    fn default() -> Self {
        Self::Mmcx
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct PhaserInitData {
    /// Timestamp of an arbitrary FastLink frame.
    ///
    /// Useful for aligning register updates to the link frames
    /// and thus achieve deterministic latency to output.
    pub frame_timestamp: i64,
}
