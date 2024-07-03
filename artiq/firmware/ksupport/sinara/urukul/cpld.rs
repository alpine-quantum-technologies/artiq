use super::{Error, SyncGen};
use crate::{rtio, spi2};
use sinara_config::urukul::{Config, Status};

type Result<T> = core::result::Result<T, Error>;

struct SpiConfig {}

impl SpiConfig {
    // SPI clock dividers for configuration register read/write.
    const DIV_CFG_WR: i32 = 2;
    const DIV_CFG_RD: i32 = 16;

    // SPI clock dividers for coarse attenuation read/write.
    const DIV_ATT_WR: i32 = 6;
    const DIV_ATT_RD: i32 = 16;

    // TODO: add clock dividers for other targets

    const FLAGS: spi2::Flags = spi2::Flags::CsPolarity;
}

#[derive(Debug)]
pub struct Cpld {
    pub bus: spi2::Bus,
    pub config: Config,
    pub sync: Option<SyncGen>,
}

impl Cpld {
    /// Initialize board-level components.
    pub fn init(&self, blind: bool) -> Result<()> {
        if !blind && !self.read_status_register()?.proto_rev_matches() {
            return Err(Error::ProtoRevMismatch);
        }

        rtio::delay_mu(100_000);

        // Pulse IO reset.
        self.write_configuration_register(Config {
            io_reset: true,
            ..self.config
        })?;
        rtio::delay_mu(100_000);
        self.write_configuration_register(Config {
            io_reset: false,
            ..self.config
        })?;

        if let Some(sync) = &self.sync {
            rtio::at_mu(rtio::now_mu() & !0xf);
            sync.device.set_mu(((1 << 4) / sync.div).into());
        }

        rtio::delay_mu(1_000_000); // DDS wake-up
        Ok(())
    }

    /// Read the status register.
    pub fn read_status_register(&self) -> Result<Status> {
        let config_reg: i32 = (&self.config).into();

        Ok(self
            .bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End | spi2::Flags::Input,
                24,
                SpiConfig::DIV_CFG_RD,
                Cs::Cfg.into(),
            )?
            .write(config_reg << 8)
            .read()
            .into())
    }

    /// Write the configuration register.
    pub fn write_configuration_register(&self, config: Config) -> Result<()> {
        let config_reg: i32 = config.into();

        self.bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End,
                24,
                SpiConfig::DIV_CFG_WR,
                Cs::Cfg.into(),
            )?
            .write(config_reg << 8);

        Ok(())
    }

    /// Write the coarse attenuation setting for all channels.
    ///
    /// # Arguments
    /// - `att` - 4 bits per channel, little-endian.
    pub fn write_attenuation_register(&self, att: u32) -> Result<()> {
        self.bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End,
                32,
                SpiConfig::DIV_ATT_WR,
                Cs::Att.into(),
            )?
            .write(att as i32);

        Ok(())
    }

    /// Read the coarse attenuation setting for all channels (4 bits per channel, little-endian).
    pub fn read_attenuation_register(&self) -> Result<u32> {
        // Shift in zeros, shift out current value, don't latch.
        self.bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::Input,
                32,
                SpiConfig::DIV_ATT_RD,
                Cs::Att.into(),
            )?
            .write(0);

        // Read the input value, shift the current value again, latch.
        let bus = self.bus.configure_mu(
            SpiConfig::FLAGS | spi2::Flags::End,
            32,
            SpiConfig::DIV_ATT_WR,
            Cs::Att.into(),
        )?;
        rtio::delay_mu(10_000);
        let data = bus.read();
        bus.write(data);

        Ok(data as u32)
    }
}

/// SPI target selection.
#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Cs {
    /// Configuration register.
    Cfg = 1,

    /// Coarse attenuators.
    Att = 2,

    /// Multiple DDS chip, as selected by `mask_nu` in the configuration register.
    DdsMulti = 3,

    /// DDS chip 0.
    DdsCh0 = 4,

    /// DDS chip 1.
    DdsCh1 = 5,

    /// DDS chip 2.
    DdsCh2 = 6,

    /// DDS chip 3.
    DdsCh3 = 7,
}
