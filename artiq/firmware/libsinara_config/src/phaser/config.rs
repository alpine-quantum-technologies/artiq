use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Config: u8 {
    /// If set, select the external SMA clock input.
    const ClkSel = 1 << 0;
    /// Active low DAC reset pin.
    const DacResetb = 1 << 1;
    /// DAC sleep pin.
    const DacSleep = 1 << 2;
    /// DAC transmission enable pin.
    const DacTxEna = 1 << 3;
    /// Quadrature upconverter 0 power save.
    const Trf0PS = 1 << 4;
    /// Quadrature upconverter 1 power save.
    const Trf1PS = 1 << 5;
    /// Active low attenuator 0 reset.
    const Att0Rstn = 1 << 6;
    /// Active low attenuator 1 reset.
    const Att1Rstn = 1 << 7;
    }
}

impl Default for Config {
    fn default() -> Self {
        // no reset, enable DAC TX
        Self::DacResetb | Self::DacTxEna | Self::Att0Rstn | Self::Att1Rstn
    }
}

impl From<Config> for u8 {
    fn from(cfg: Config) -> Self {
        cfg.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let cfg = Config::default();
        assert_eq!(cfg.bits(), 0b11001010);
    }

    #[test]
    fn clear_bits() {
        let cfg = Config::default() & !Config::DacTxEna;
        assert!(!cfg.contains(Config::DacTxEna));
        assert_eq!(cfg.bits(), 0b11000010);
    }
}
