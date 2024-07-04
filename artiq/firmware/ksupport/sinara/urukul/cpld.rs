use super::{ad9910::Ad9910, Channel, Cs, Error, Result, SpiConfig, SyncGen};
use crate::{rtio, sinara::ttl, spi2};
use sinara_config::urukul::{Config, Status};

#[derive(Debug)]
pub struct Cpld {
    pub bus: spi2::Bus,
    pub config: Config,
    pub sync: Option<SyncGen>,
    pub io_update: Option<ttl::TtlOut>,
    pub channels: [ChannelDesc; 4],
}

#[derive(Debug)]
pub struct ChannelDesc {
    pub switch_device: ttl::TtlOut,
    pub pll_cp: ad9910_pac::cfr3::ICpA,
    pub pll_vco: ad9910_pac::cfr3::VcoSelA,
    pub pll_n: u8,
    pub pll_en: bool,
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

        // Initialize channels.
        self.channel(Channel::Zero).init(blind)?;
        self.channel(Channel::One).init(blind)?;
        self.channel(Channel::Two).init(blind)?;
        self.channel(Channel::Three).init(blind)?;

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

    /// Pulse the I/O update signal on all DDS channels.
    pub fn pulse_io_update(&self, duration: i64) -> Result<()> {
        if let Some(ttl) = &self.io_update {
            ttl.pulse_mu(duration);
            Ok(())
        } else {
            self.write_configuration_register(Config {
                io_update: true,
                ..self.config
            })?;
            rtio::delay_mu(duration);
            self.write_configuration_register(Config {
                io_update: false,
                ..self.config
            })
        }
    }

    /// Proxy to a AD9910 channel.
    pub fn channel(&self, channel: Channel) -> Ad9910<'_> {
        let index: usize = channel.into();
        Ad9910 {
            channel,
            cpld: self,
            config: &self.channels[index],
        }
    }
}
