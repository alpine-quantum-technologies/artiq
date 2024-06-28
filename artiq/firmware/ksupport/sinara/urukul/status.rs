#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Status {
    // TODO: add rf_switch
    // TODO: add smp_error
    // TODO: add pll_lock
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

impl Status {
    #[allow(dead_code)]
    const RF_SW: u8 = 0;
    #[allow(dead_code)]
    const SMP_ERR: u8 = 4;
    #[allow(dead_code)]
    const PLL_LOCK: u8 = 8;
    const IFC_MODE: u8 = 12;
    const PROTO_REV: u8 = 16;

    /// True if the protocol revision is supported by this driver.
    pub fn proto_rev_matches(&self) -> bool {
        self.proto_rev == 0x08
    }
}

impl From<i32> for Status {
    fn from(value: i32) -> Self {
        Self {
            ifc_mode: IfcMode::from_bits_truncate(((value >> Self::IFC_MODE) & 0xf) as u8),
            proto_rev: ((value >> Self::PROTO_REV) & 0x7f) as u8,
        }
    }
}
