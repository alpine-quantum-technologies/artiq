use bitflags::bitflags;
use num_enum::IntoPrimitive;

bitflags! {
    /// SPI configuration flags.
    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct SpiConfig: u8 {
    const Offline = 1 << 0;
    const End = 1 << 1;
    const ClkPhase = 1 << 2;
    const ClkPolartiy = 1 << 3;
    const HalfDuplex = 1 << 4;
    const LsbFirst = 1 << 5;
    }
}

impl Default for SpiConfig {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<SpiConfig> for u8 {
    fn from(cfg: SpiConfig) -> Self {
        cfg.bits()
    }
}

/// SPI chip select.
#[derive(Debug, IntoPrimitive)]
#[repr(u8)]
pub enum SpiCs {
    /// Select the DAC.
    Dac = 1 << 0,
    /// Select the quadrature upconverter 0.
    Trf0 = 1 << 1,
    /// Select the quadrature upconverter 1.
    Trf1 = 1 << 2,
    /// Select the attenuator 0.
    Att0 = 1 << 3,
    /// Select the attenuator 1.
    Att1 = 1 << 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let cfg = SpiConfig::default();
        assert!(cfg.is_empty());

        let reg: u8 = cfg.into();
        assert_eq!(reg, 0);
    }

    #[test]
    fn spi_end() {
        let cfg = SpiConfig::default() | SpiConfig::End;
        let reg: u8 = cfg.into();
        assert_eq!(reg, 0b10);
    }
}
