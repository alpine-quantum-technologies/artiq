use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Status: u8 {
    /// DAC alarm pin.
    const DacAlarm = 1;

    /// Quadrature upconverter 0 lock detect.
    const Trf0LD = 2;

    /// Quadrature upconverter 1 lock detect.
    const Trf1LD = 4;

    /// ADC channel 0 termination indicator.
    const Term0 = 8;

    /// ADC channel 1 termination indicator.
    const Term1 = 16;

    /// SPI machine is idle and data registers can be read/written.
    const SpiIdle = 32;
    }
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        let status: Status = 0.into();
        assert!(status.is_empty());

        let status: Status = 0b110.into();
        assert!(!status.is_empty());
        assert!(!status.is_all());
        assert!(status.contains(Status::Trf0LD));
        assert!(status.contains(Status::Trf1LD));
    }
}
