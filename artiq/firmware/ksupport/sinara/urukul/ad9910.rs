use super::{cpld::ChannelDesc, Channel, Cpld, Cs, Error, Result, SpiConfig, SyncData};
use crate::{rtio, spi2};

/// Urukul AD9910 channel.
pub struct Ad9910<'a> {
    /// Channel index.
    pub channel: Channel,

    /// Urukul board controller.
    pub cpld: &'a Cpld,

    /// Configuration data
    pub config: &'a ChannelDesc,
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

        unsafe {
            ad9910_pac::regs(self).cfr2().write_with_zero(|w| {
                w.enable_amplitude_scale_from_single_tone_profiles()
                    .set_bit()
                    .read_effective_ftw()
                    .set_bit()
                    .sync_timing_validation_disable()
                    .set_bit()
            })?;
        }
        self.cpld.pulse_io_update(1_000)?;

        let pll_vco = self.config.pll_vco;
        let pll_cp = self.config.pll_cp;
        let pll_n = self.config.pll_n;
        let pll_en = self.config.pll_en;

        ad9910_pac::regs(self).cfr3().write(|w| {
            w.refclk_input_divider_bypass()
                .set_bit()
                .vco_sel()
                .variant(pll_vco)
                .i_cp()
                .variant(pll_cp)
                .n()
                .variant(pll_n)
                .pll_enable()
                .bit(pll_en)
                .pfd_reset()
                .set_bit()
                .drv0()
                .variant(ad9910_pac::regs::cfr3::Drv0A::Disabled)
        })?;
        self.cpld.pulse_io_update(1_000)?;

        ad9910_pac::regs(self)
            .cfr3()
            .modify(|_, w| w.pfd_reset().clear_bit())?;
        self.cpld.pulse_io_update(1_000)?;

        // Wait for PLL lock, up to 100 ms.
        for i in 0..100 {
            let sta = self.cpld.read_status_register()?;
            rtio::delay_mu(1_000_000);

            if sta.pll_locked(self.channel.into()) {
                break;
            }

            if i >= 99 {
                return Err(Error::PllNotLocked(self.channel));
            }
        }
        rtio::delay_mu(10_000); // slack

        Ok(())
    }

    pub fn read_sync_data(&self) -> Result<SyncData> {
        let sync_data = SyncData::init(self.config.sync_data_source, &self.cpld.i2c_bus)?;
        rtio::delay_mu(50_000_000);
        Ok(sync_data)
    }

    pub fn setup_sync(&self, data: &SyncData) -> Result<()> {
        self.set_sync(data.sync_delay_seed, data.validation_window)?;
        self.clear_smp_err()?;
        rtio::delay_mu(10_000_000); // slack
        Ok(())
    }

    /// Enable the RF output (close the switch).
    #[inline(always)]
    pub fn switch_on(&self) {
        self.config.switch_device.on()
    }

    /// Disable the RF output (open the switch).
    #[inline(always)]
    pub fn switch_off(&self) {
        self.config.switch_device.off()
    }

    /// Single-tone output update, on the default profile.
    pub fn set_mu(&self, ftw: u32, pow: u16, asf: u16) -> Result<u16> {
        ad9910_pac::regs(self)
            .single_tone_profile7() // TODO: obey Config::profile.
            .write(|w| unsafe { w.asf().bits(asf).pow().bits(pow).ftw().bits(ftw) })?;
        self.cpld.pulse_io_update(8)?;

        Ok(pow)
    }

    pub fn set_mu_coherent(
        &self,
        ftw: u32,
        pow: u16,
        asf: u16,
        ref_time_mu: i64,
        io_update_delay_mu: i64,
    ) -> Result<u16> {
        rtio::at_mu(rtio::now_mu() & !7);
        // Use write, not modify, to avoid reading back the register.
        ad9910_pac::regs(self).cfr1().write(|w| {
            w.sdio_input_only()
                .set_bit()
                .autoclear_phase_accumulator()
                .set_bit()
        })?;

        let sysclk_per_mu = 1; // TODO: calculate this elsewhere.
        let dt = (rtio::now_mu() as i32) - (ref_time_mu as i32);
        let pow = (pow as i32) + ((dt * (ftw as i32) * sysclk_per_mu) >> 16);
        let pow = (pow & 0xffff) as u16;

        ad9910_pac::regs(self)
            .single_tone_profile7()
            .write(|w| unsafe { w.asf().bits(asf).pow().bits(pow).ftw().bits(ftw) })?;

        rtio::delay_mu(io_update_delay_mu);
        self.cpld.pulse_io_update(8)?;

        rtio::at_mu(rtio::now_mu() & !7);
        ad9910_pac::regs(self)
            .cfr1()
            .write(|w| w.sdio_input_only().set_bit())?;
        Ok(pow)
    }

    fn set_sync(&self, sync_delay: u8, window: u8) -> Result<()> {
        use ad9910_pac::regs::multichip_sync::SyncGeneratorPolarityA::RisingEdge;

        ad9910_pac::regs(self).multichip_sync().write(|w| unsafe {
            w.sync_validation_delay()
                .bits(window)
                .sync_receiver_enable()
                .set_bit()
                .sync_generator_enable()
                .clear_bit()
                .sync_generator_polarity()
                .variant(RisingEdge)
                .sync_state_preset_value()
                .bits(0)
                .output_sync_generator_delay()
                .bits(0)
                .input_sync_receiver_delay()
                .bits(sync_delay)
        })?;
        Ok(())
    }

    fn clear_smp_err(&self) -> Result<()> {
        ad9910_pac::regs(self)
            .cfr2()
            .write(|w| w.sync_timing_validation_disable().set_bit())?;
        self.cpld.pulse_io_update(1_000)?;
        rtio::delay_mu(10_000); // slack
        unsafe {
            ad9910_pac::regs(self).cfr2().write_with_zero(|w| {
                w.enable_amplitude_scale_from_single_tone_profiles()
                    .set_bit()
                    .read_effective_ftw()
                    .set_bit()
                    .sync_timing_validation_disable()
                    .clear_bit()
            })?;
        }
        rtio::delay_mu(10_000); // slack
        self.cpld.pulse_io_update(1_000)
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

        let val = self
            .cpld
            .bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End | spi2::Flags::Input,
                16,
                SpiConfig::DIV_DDS_RD,
                cs.into(),
            )?
            .write(0)
            .read() as u16;

        rtio::delay_mu(10_000); // slack
        Ok(val)
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
        let val = self
            .cpld
            .bus
            .configure_mu(
                SpiConfig::FLAGS | spi2::Flags::End | spi2::Flags::Input,
                32,
                SpiConfig::DIV_DDS_RD,
                cs.into(),
            )?
            .write(0)
            .read() as u32;

        rtio::delay_mu(10_000); // slack
        Ok(val)
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

        rtio::delay_mu(10_000); // slack

        Ok((hi << 32) | lo)
    }
}
