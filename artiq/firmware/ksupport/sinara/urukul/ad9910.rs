use super::{Channel, Cpld, Cs, Error, Result, SpiConfig};
use crate::{rtio, sinara::ttl, spi2};

/// Urukul AD9910 channel.
pub struct Ad9910<'a> {
    /// Channel index.
    pub channel: Channel,

    /// Urukul board controller.
    pub cpld: &'a Cpld,

    /// EEM switch line driver.
    pub switch_device: &'a ttl::TtlOut,
}

impl Ad9910<'_> {
    pub fn init(&self, blind: bool) -> Result<()> {
        ad9910_pac::regs(self)
            .cfr1()
            .write(|w| w.sdio_input_only().set_bit().lsb_first().clear_bit())?;

        self.cpld.pulse_io_update(1_000)?;
        rtio::delay_mu(1_000_000);

        if !blind {
            let aux_dac_fsc = ad9910_pac::regs(self)
                .aux_dac_control()
                .read()?
                .fsc()
                .bits();
            if aux_dac_fsc != 0x7f {
                // not the reset value
                return Err(Error::Ad9910AuxDacMismatch);
            }
            rtio::delay_mu(50_000); // slack
        }

        Ok(())
    }

    /// Enable the RF output (close the switch).
    pub fn switch_on(&self) {
        self.switch_device.on()
    }

    /// Disable the RF output (open the switch).
    pub fn switch_off(&self) {
        self.switch_device.off()
    }
}

impl ad9910_pac::Interface<u16> for Ad9910<'_> {
    type Error = Error;

    fn write(&self, addr: u8, data: u16) -> Result<()> {
        let cs: Cs = self.channel.into();
        let addr: i32 = addr.into();

        self.cpld
            .bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End,
                24,
                SpiConfig::DIV_DDS_WR,
                cs.into(),
            )?
            .write((addr << 24) | ((data as i32) << 8));

        Ok(())
    }

    fn read(&self, addr: u8) -> Result<u16> {
        let cs: Cs = self.channel.into();
        let addr: i32 = addr.into();

        self.cpld
            .bus
            .configure_mu(SpiConfig::FLAGS, 8, SpiConfig::DIV_DDS_WR, cs.into())?
            .write((addr | 0x80) << 24);

        Ok(self
            .cpld
            .bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End | spi2::Flags::Input,
                16,
                SpiConfig::DIV_DDS_RD,
                cs.into(),
            )?
            .write(0)
            .read() as u16)
    }
}

impl ad9910_pac::Interface<u32> for Ad9910<'_> {
    type Error = Error;

    fn write(&self, addr: u8, data: u32) -> Result<()> {
        let cs: Cs = self.channel.into();
        let addr: i32 = addr.into();

        self.cpld
            .bus
            .configure_mu(SpiConfig::FLAGS, 8, SpiConfig::DIV_DDS_WR, cs.into())?
            .write(addr << 24);
        self.cpld
            .bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End,
                32,
                SpiConfig::DIV_DDS_WR,
                cs.into(),
            )?
            .write(data as i32);

        Ok(())
    }

    fn read(&self, addr: u8) -> Result<u32> {
        let cs: Cs = self.channel.into();
        let addr: i32 = addr.into();

        self.cpld
            .bus
            .configure_mu(SpiConfig::FLAGS, 8, SpiConfig::DIV_DDS_WR, cs.into())?
            .write((addr | 0x80) << 24);
        Ok(self
            .cpld
            .bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End | spi2::Flags::Input,
                32,
                SpiConfig::DIV_DDS_RD,
                cs.into(),
            )?
            .write(0)
            .read() as u32)
    }
}

impl ad9910_pac::Interface<u64> for Ad9910<'_> {
    type Error = Error;

    fn write(&self, addr: u8, data: u64) -> Result<()> {
        let cs: Cs = self.channel.into();
        let addr: i32 = addr.into();

        self.cpld
            .bus
            .configure_mu(SpiConfig::FLAGS, 8, SpiConfig::DIV_DDS_WR, cs.into())?
            .write(addr << 24);
        self.cpld
            .bus
            .configure_mu(SpiConfig::FLAGS, 32, SpiConfig::DIV_DDS_WR, cs.into())?
            .write((data >> 32) as i32);
        self.cpld
            .bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End,
                32,
                SpiConfig::DIV_DDS_WR,
                cs.into(),
            )?
            .write((data & 0xffffffff) as i32);

        Ok(())
    }

    fn read(&self, addr: u8) -> Result<u64> {
        let cs: Cs = self.channel.into();
        let addr: i32 = addr.into();

        self.cpld
            .bus
            .configure_mu(SpiConfig::FLAGS, 8, SpiConfig::DIV_DDS_WR, cs.into())?
            .write((addr | 0x80) << 24);

        let bus_hi = self.cpld.bus.configure_mu(
            SpiConfig::FLAGS | spi2::Flags::Input,
            32,
            SpiConfig::DIV_DDS_RD,
            cs.into(),
        )?;
        bus_hi.write(0);

        let bus_lo = self.cpld.bus.configure_mu(
            SpiConfig::FLAGS | spi2::Flags::End | spi2::Flags::Input,
            32,
            SpiConfig::DIV_DDS_RD,
            cs.into(),
        )?;
        bus_lo.write(0);

        let hi = bus_hi.read() as u64;
        let lo = bus_lo.read() as u64;

        Ok((hi << 32) | lo)
    }
}
