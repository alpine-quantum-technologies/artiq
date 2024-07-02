use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::Deserialize_repr;

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
