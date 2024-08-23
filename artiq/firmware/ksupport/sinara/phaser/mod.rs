use crate::rtio;
use crate::RTIO_OUTPUT_FN as rtio_output;
use proto_artiq::kernel_proto::Message::*;
use sinara_config::phaser::{
    Address, Config, PhaserInitData, SpiConfig, SpiCs, Status, BOARD_ID, DAC_MMAP,
};

use core::format_args;

#[derive(Debug)]
pub struct Phaser {
    pub channel_base: i32,
    pub miso_delay: i32,
    pub sync_dly: u8,
    pub tune_fifo_offset: bool,
}

fn break_realtime() {
    rtio::at_mu(rtio::get_counter() + 125_000);
}

impl Phaser {
    const T_FRAME: i64 = 10 * 8 * 4;

    pub fn init(&self, debug: bool) -> PhaserInitData {
        let board_id = self.read8(Address::BoardId);
        assert_eq!(board_id, BOARD_ID);
        rtio::delay_mu(100_000); // slack

        let hw_rev = self.read8(Address::HwRev);
        rtio::delay_mu(100_000); // slack
        let is_baseband = hw_rev & (1 << 4) == (1 << 4);

        let gw_rev = self.read8(Address::GwRev);
        assert_eq!(gw_rev, 1); // base, not miqro
        rtio::delay_mu(100_000); // slack

        let frame_timestamp = self.measure_frame_timestamp();

        if debug {
            let crc_err = self.get_crc_err();
            rtio::delay_mu(100_000);

            crate::send(&Log(format_args!(
                "Phaser: hw_rev={}, gw_rev={}, is_baseband={}, crc_err={}\n",
                hw_rev, gw_rev, is_baseband, crc_err
            )));
            rtio::at_mu(rtio::get_counter() + 125_000);
        }

        // Reset
        self.set_cfg(Config::DacSleep | Config::Trf0PS | Config::Trf1PS);
        self.set_leds(0);
        self.set_fan_mu(0);

        // Bring DAC out of reset, keep TX off
        self.set_cfg(
            Config::default() & !Config::DacTxEna & !Config::Att0Rstn & !Config::Att1Rstn
                | Config::Trf0PS
                | Config::Trf1PS,
        ); // TODO: clk_sel
        rtio::delay_mu(100_000); // slack

        self.set_sync_dly(self.sync_dly);

        // 4 wire SPI, sif4_enable
        self.dac_write(0x02, 0x0080);

        assert_eq!(
            self.dac_read(0x7f, 34),
            0x5409,
            "DAC version readback invalid"
        );
        rtio::delay_mu(100_000); // slack
        assert_eq!(
            self.dac_read(0x00, 34),
            0x049c,
            "DAC config0 readback invalid"
        );
        rtio::delay_mu(100_000); // slack

        let dac_temperature = self.get_dac_temperature();
        if debug {
            crate::send(&Log(format_args!("dac_temperature={}\n", dac_temperature)));
            break_realtime();
        }
        assert!(
            dac_temperature > 10 && dac_temperature < 90,
            "DAC temperature out of bounds"
        );

        for data in DAC_MMAP {
            self.dac_write((data >> 16) as u8, data as u16);
            rtio::delay_mu(120_000);
        }
        self.dac_sync();
        rtio::delay_mu(40_000); // slack

        // pll_ndivsync_ena disable
        let config18 = self.dac_read(0x18, 34);
        rtio::delay_mu(100_000); // slack
        self.dac_write(0x18, config18 & !0x0800);

        let fifo_offset = if self.tune_fifo_offset {
            self.dac_tune_fifo_offset()
        } else {
            0
        };

        if debug {
            crate::send(&Log(format_args!("fifo_offset={}\n", fifo_offset)));
            break_realtime();
        }

        self.clear_dac_alarms();
        rtio::delay_mu(2_000_000); // let it run a bit
        let alarms = self.get_dac_alarms();
        rtio::delay_mu(100_000); // slack
        if debug && (alarms & !0x0040) != 0 {
            // ignore PLL alarms (see DS)
            crate::send(&Log(format_args!("DAC alarms={}\n", alarms)));
            break_realtime();
        }

        // Avoid malformed output for: mixer_ena=1, nco_ena=0 after power up
        self.dac_write((DAC_MMAP[2] >> 16) as u8, (DAC_MMAP[2] as u16) | (1 << 4));
        rtio::delay_mu(40_000);
        self.dac_sync();
        rtio::delay_mu(100_000);
        self.dac_write((DAC_MMAP[2] >> 16) as u8, DAC_MMAP[2] as u16);
        rtio::delay_mu(40_000);
        self.dac_sync();
        rtio::delay_mu(100_000);

        // Power up upconverters, release attenuators reset.
        self.set_cfg(Config::default() & !Config::DacTxEna); // TODO: clk_sel

        // TODO: configure channel attenuation
        // TODO: enable DAC Tx

        PhaserInitData { frame_timestamp }
    }

    #[inline]
    fn write8(&self, addr: Address, data: u8) {
        let addr: u8 = addr.into();
        unsafe {
            rtio_output(
                (self.channel_base << 8) | (addr & 0x7f) as i32 | 0x80,
                data as i32,
            )
        };
        rtio::delay_mu(Self::T_FRAME);
    }

    #[inline]
    fn read8(&self, addr: Address) -> u8 {
        let addr: u8 = addr.into();
        unsafe { rtio_output((self.channel_base << 8) | (addr & 0x7f) as i32, 0) };
        let response = rtio::input_data(self.channel_base);
        ((response >> self.miso_delay) & 0xff) as u8
    }

    #[inline]
    fn spi_cfg(&self, select: SpiCs, div: u8, length: u8, config: SpiConfig) {
        self.write8(Address::SpiSel, select.into());
        self.write8(Address::SpiDivLen, ((div - 2) >> 3) | ((length - 1) << 5));
        self.write8(Address::SpiCfg, config.into());
    }

    #[inline]
    fn spi_write(&self, data: u8) {
        self.write8(Address::SpiDatW, data);
        crate::send(&Log(format_args!("spi_write: {}\n", data)));
        break_realtime();
    }

    #[inline]
    fn spi_read(&self) -> u8 {
        let val = self.read8(Address::SpiDatR);
        crate::send(&Log(format_args!("spi_read: {}\n", val)));
        break_realtime();
        val
    }

    fn dac_write(&self, addr: u8, data: u16) {
        let div = 34; // 100 ns min period
        let t_xfer = (8 + 1) * (div as i64) * 4;
        self.spi_cfg(SpiCs::Dac, div, 8, SpiConfig::default());
        self.spi_write(addr);
        rtio::delay_mu(t_xfer);
        self.spi_write((data >> 8) as u8);
        rtio::delay_mu(t_xfer);
        self.spi_cfg(SpiCs::Dac, div, 8, SpiConfig::default() | SpiConfig::End);
        self.spi_write(data as u8);
        rtio::delay_mu(t_xfer);
    }

    fn dac_read(&self, addr: u8, div: u8) -> u16 {
        let t_xfer = ((8 + 1) * div * 4) as i64;
        self.spi_cfg(SpiCs::Dac, div, 8, SpiConfig::default());
        self.spi_write(addr | 0x80);
        rtio::delay_mu(t_xfer);
        self.spi_write(0);
        rtio::delay_mu(t_xfer);
        let mut data = (self.spi_read() as u16) << 8;
        rtio::delay_mu(20_000); // slack
        self.spi_cfg(SpiCs::Dac, div, 8, SpiConfig::default() | SpiConfig::End);
        self.spi_write(0);
        rtio::delay_mu(t_xfer);
        data |= self.spi_read() as u16;
        data
    }

    fn dac_sync(&self) {
        let config1f = self.dac_read(0x1f, 34);
        rtio::delay_mu(400_000);
        self.dac_write(0x1f, config1f & !2);
        self.dac_write(0x1f, config1f | 2);
    }

    /// Read the CRC error counter.
    ///
    /// The counter overflows at 256 and resets on device reset.
    #[inline]
    fn get_crc_err(&self) -> u8 {
        self.read8(Address::CrcErr)
    }

    /// Read the status register.
    #[inline]
    fn get_sta(&self) -> Status {
        self.read8(Address::Sta).into()
    }

    /// Write the configuration register.
    #[inline]
    fn set_cfg(&self, cfg: Config) {
        self.write8(Address::Cfg, cfg.into())
    }

    /// Set the front panel LEDs.
    #[inline]
    fn set_leds(&self, leds: u8) {
        self.write8(Address::Led, leds & 0x3f)
    }

    /// Set the fan duty cycle.
    #[allow(dead_code)]
    #[inline]
    fn set_fan_mu(&self, pwm: u8) {
        self.write8(Address::Fan, pwm)
    }

    /// Read the DAC die temperature, in degree Celsius.
    #[inline]
    fn get_dac_temperature(&self) -> i32 {
        (self.dac_read(0x06, 255) >> 8).into()
    }

    /// Measure the timestamp of an arbitrary frame.
    fn measure_frame_timestamp(&self) -> i64 {
        unsafe { rtio_output(self.channel_base << 8, 0) }; // read any register
        let frame_timestamp =
            rtio::input_timestamp(rtio::now_mu() + 4 * Self::T_FRAME, self.channel_base);
        rtio::delay_mu(100_000);
        frame_timestamp
    }

    #[inline]
    fn set_sync_dly(&self, dly: u8) {
        self.write8(Address::SyncDly, dly & 0b111)
    }

    #[inline]
    fn get_dac_alarms(&self) -> u16 {
        self.dac_read(0x05, 34)
    }

    #[inline]
    fn clear_dac_alarms(&self) {
        self.dac_write(0x05, 0)
    }

    /// Scan through `fifo_offset` and configure midpoint setting.
    fn dac_tune_fifo_offset(&self) -> u16 {
        let config9 = self.dac_read(0x09, 34);
        rtio::delay_mu(100_000); // slack

        let mut good = 0;
        for o in 0..8 {
            self.dac_write(0x09, (config9 & 0x1fff) | (o << 13));
            self.clear_dac_alarms();
            rtio::delay_mu(100_000); // run
            let alarms = self.get_dac_alarms();
            rtio::delay_mu(100_000); // slack
            if (alarms >> 11) & 0x7 == 0 {
                // any fifo alarm
                good |= 1 << o;
            }
        }

        // if there are good offsets accross the wrap around
        // offset for computations
        let offset = if good & 0x81 == 0x81 {
            good = ((good << 4) & 0xf0) | (good >> 4);
            4
        } else {
            0
        };

        // calculate mean
        let mut sum = 0;
        let mut count = 0;
        for o in 0..8 {
            if good & (1 << o) == (1 << o) {
                sum += o;
                count += 1;
            }
        }
        assert_ne!(count, 0, "no good fifo offset");
        let best = ((sum / count) + offset) % 8;
        self.dac_write(0x09, (config9 & 0x1fff) | (best << 13));
        best
    }
}
