use super::Cpld;
use crate::sinara::ttl;

/// Urukul AD9910 channel.
pub struct Ad9910<'a> {
    /// Urukul board controller.
    pub cpld: &'a Cpld,

    /// EEM switch line driver.
    pub switch_device: &'a ttl::TtlOut,
}

impl Ad9910<'_> {
    /// Enable the RF output (close the switch).
    pub fn switch_on(&self) {
        self.switch_device.on()
    }

    /// Disable the RF output (open the switch).
    pub fn switch_off(&self) {
        self.switch_device.off()
    }
}
