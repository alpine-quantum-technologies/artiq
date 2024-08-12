use crate::rtio;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone)]
    pub struct Flags: u8 {
    const Offline = 0x01;
    const End = 0x02;
    const Input = 0x04;
    const CsPolarity = 0x08;
    const ClkPolarity = 0x10;
    const ClkPhase = 0x20;
    const LsbFirst = 0x40;
    const HalfDuplex = 0x80;
    }
}

#[derive(Debug)]
pub struct Bus {
    pub channel: i32,
    pub ref_period_mu: i64,
}

#[derive(Debug)]
pub struct ConfiguredBus {
    channel: i32,
    xfer_duration_mu: i64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    InvalidTransferLength(i32),
    InvalidClockDivider(i32),
}

impl Bus {
    /// Offset of the configuration register in the RTIO PHY, relative to the base RTIO channel.
    const CONFIG_ADDR: i32 = 1;

    /// Offset of the data register in the RTIO PHY, relative to the base RTIO channel.
    const DATA_ADDR: i32 = 0;

    /// Configure the RTIO PHY for upcoming transactions.
    ///
    /// # Arguments
    ///
    /// * `flags` - SPI transaction flags.
    /// * `length` - SPI transfer length. Must be between 1 and 32 (included).
    /// * `div` - SPI clock divider. The base clock is the RTIO clock. Must be between 2 and 257 (included).
    /// * `cs` - SPI chip select.
    pub fn configure_mu(
        &self,
        flags: Flags,
        length: i32,
        div: i32,
        cs: u8,
    ) -> Result<ConfiguredBus, Error> {
        if !(1..=32).contains(&length) {
            return Err(Error::InvalidTransferLength(length));
        }

        if !(2..=257).contains(&div) {
            return Err(Error::InvalidClockDivider(div));
        }

        unsafe {
            crate::RTIO_OUTPUT_FN(
                (self.channel << 8) | Self::CONFIG_ADDR,
                (flags.bits() as i32)
                    | ((length - 1) << 8)
                    | ((div - 2) << 16)
                    | ((cs as i32) << 24),
            );
        }

        rtio::delay_mu(self.ref_period_mu);

        Ok(ConfiguredBus {
            channel: self.channel,
            xfer_duration_mu: xfer_duration_mu(div, length, self.ref_period_mu),
        })
    }
}

impl ConfiguredBus {
    /// Write to the SPI bus.
    ///
    /// The length of the written data is determined by the bus configuration.
    ///
    /// The timeline cursor is advanced by the transfer duration.
    pub fn write(&self, data: i32) -> &Self {
        unsafe { crate::RTIO_OUTPUT_FN((self.channel << 8) | Bus::DATA_ADDR, data) };
        rtio::delay_mu(self.xfer_duration_mu);
        self
    }

    /// Read from the SPI bus.
    ///
    /// The length of the read data is determined by the bus configuration.
    ///
    /// This function blocks until data is available.
    pub fn read(&self) -> i32 {
        rtio::input_data(self.channel)
    }
}

fn xfer_duration_mu(div: i32, length: i32, ref_multiplier: i64) -> i64 {
    (((length as i64) + 1) * (div as i64) + 1) * ref_multiplier
}
