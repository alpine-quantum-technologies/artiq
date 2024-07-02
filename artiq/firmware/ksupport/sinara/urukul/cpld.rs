use super::{Error, SyncGen};
use crate::{rtio, spi2};
use sinara_config::urukul::{Config, Status};

type Result<T> = core::result::Result<T, Error>;

struct SpiConfig {}

impl SpiConfig {
    // SPI clock dividers for configuration register read/write.
    const DIV_CFG_WR: i32 = 2;
    const DIV_CFG_RD: i32 = 16;

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
