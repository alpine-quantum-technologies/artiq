use bitflags::bitflags;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Status {
    /// RF output switch status. Flag set means switch closed (output enabled).
    pub rf_sw: ChannelFlags,

    /// DDS chips sampling error. Flag set means error.
    pub smp_err: ChannelFlags,

    /// DDS chips PLL lock status. Flag set means PLL locked.
    pub pll_lock: ChannelFlags,

    /// DIP-switch setting.
    pub ifc_mode: IfcMode,

    /// Protocol revision, 7 bits.
    pub proto_rev: u8,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct IfcMode: u8 {
    /// Whether the AD9910 variant is populated.
    const En9910 = 0x01;
    /// Whether the NU-Servo (SU-Servo) mode is used.
    const EnNu = 0x02;
    /// Whether the SYNC signals on EEM1 must be driven.
    const EnEem1 = 0x04;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ChannelFlags: u8 {
    const Zero = 0;
    const One = 1;
    const Two = 2;
    const Three = 3;
    }
}

impl Status {
    const RF_SW: u8 = 0;
    const SMP_ERR: u8 = 4;
    const PLL_LOCK: u8 = 8;
    const IFC_MODE: u8 = 12;
    const PROTO_REV: u8 = 16;

    /// True if the protocol revision is supported by this driver.
    pub fn proto_rev_matches(&self) -> bool {
        self.proto_rev == 0x08
    }

    /// True if all PLLs are locked.
    pub fn all_plls_locked(&self) -> bool {
        self.pll_lock.is_all()
    }

    /// True if any channel has the sampling error flag set.
    pub fn any_sampling_error(&self) -> bool {
        self.smp_err.bits() != 0
    }
}

impl From<i32> for Status {
    fn from(value: i32) -> Self {
        Self {
            rf_sw: ChannelFlags::from_bits_truncate(((value >> Self::RF_SW) & 0xf) as u8),
            smp_err: ChannelFlags::from_bits_truncate(((value >> Self::SMP_ERR) & 0xf) as u8),
            pll_lock: ChannelFlags::from_bits_truncate(((value >> Self::PLL_LOCK) & 0xf) as u8),
            ifc_mode: IfcMode::from_bits_truncate(((value >> Self::IFC_MODE) & 0xf) as u8),
            proto_rev: ((value >> Self::PROTO_REV) & 0x7f) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_zero() {
        let status: Status = 0.into();
        assert_eq!(status.rf_sw.bits(), 0);
        assert_eq!(status.smp_err.bits(), 0);
        assert_eq!(status.pll_lock.bits(), 0);
        assert!(!status.all_plls_locked());
        assert!(!status.proto_rev_matches());
    }

    #[test]
    fn realistic() {
        let status = Status {
            rf_sw: ChannelFlags::from_bits_truncate(0b1011),
            smp_err: ChannelFlags::from_bits_truncate(0),
            pll_lock: ChannelFlags::from_bits_truncate(0xf),
            ifc_mode: IfcMode::from_bits_truncate(0),
            proto_rev: 8,
        };

        assert!(status.proto_rev_matches());
        assert!(status.all_plls_locked());
        assert!(!status.any_sampling_error());

        let from_i32: Status = 0x80f0b.into();
        assert_eq!(from_i32, status);
    }

    #[test]
    fn with_sampling_error() {
        let status: Status = 0x80f20.into();
        assert!(status.any_sampling_error());
        assert!(status.smp_err.intersects(ChannelFlags::Two));
    }
}
