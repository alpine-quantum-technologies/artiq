use super::{ClkDiv, ClkSel, SyncSel};
use core::convert::{TryFrom, TryInto};

/// Urukul configuration register.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Config {
    // TODO: add RF switches and LED configuration.
    pub profile: u8,
    pub io_update: bool,
    // TODO: add mask_nu
    pub clk_sel: ClkSel,
    pub sync_sel: SyncSel,
    pub reset: bool,
    pub io_reset: bool,
    pub clk_div: ClkDiv,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InvalidConfig {
    ClkSel(i32),
    SyncSel(i32),
    ClkDiv(i32),
}

impl Config {
    // Field offsets in the configuration register.
    #[allow(dead_code)]
    const RF_SW: u8 = 0;
    #[allow(dead_code)]
    const LED: u8 = 4;
    const PROFILE: u8 = 8;
    const IO_UPDATE: u8 = 12;
    #[allow(dead_code)]
    const MASK_NU: u8 = 13;
    const CLK_SEL0: u8 = 17;
    const SYNC_SEL: u8 = 18;
    const RST: u8 = 19;
    const IO_RST: u8 = 20;
    const CLK_SEL1: u8 = 21;
    const CLK_DIV: u8 = 22;
}

impl TryFrom<i32> for Config {
    type Error = InvalidConfig;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let clk_sel0 = (value >> Self::CLK_SEL0) & 1;
        let clk_sel1 = (value >> Self::CLK_SEL1) & 1;
        let clk_sel = clk_sel0 | (clk_sel1 << 1);

        let sync_sel = (value >> Self::SYNC_SEL) & 1;
        let clk_div = (value >> Self::CLK_DIV) & 3;

        Ok(Self {
            profile: ((value >> Self::PROFILE) & 7) as u8,
            io_update: ((value >> Self::IO_UPDATE) & 1) != 0,
            clk_sel: ClkSel::try_from(clk_sel as u8).map_err(|_| Self::Error::ClkSel(clk_sel))?,
            sync_sel: SyncSel::try_from(sync_sel as u8)
                .map_err(|_| Self::Error::SyncSel(sync_sel))?,
            reset: ((value >> Self::RST) & 1) != 0,
            io_reset: ((value >> Self::IO_RST) & 1) != 0,
            clk_div: ClkDiv::try_from(clk_div as u8).map_err(|_| Self::Error::ClkDiv(clk_div))?,
        })
    }
}

impl From<Config> for i32 {
    fn from(config: Config) -> Self {
        (&config).into()
    }
}

impl From<&Config> for i32 {
    fn from(config: &Config) -> Self {
        let clk_sel: u8 = config.clk_sel.into();
        let sync_sel: u8 = config.sync_sel.into();
        let clk_div: u8 = config.clk_div.into();

        ((config.profile as i32) << Config::PROFILE)
            | (bool_to_i32(config.io_update) << Config::IO_UPDATE)
            | (((clk_sel & 1) as i32) << Config::CLK_SEL0)
            | (((sync_sel & 1) as i32) << Config::SYNC_SEL)
            | (bool_to_i32(config.reset) << Config::RST)
            | (bool_to_i32(config.io_reset) << Config::IO_RST)
            | ((((clk_sel >> 1) & 1) as i32) << Config::CLK_SEL1)
            | ((clk_div as i32) << Config::CLK_DIV)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profile: 7,
            clk_sel: ClkSel::default(),
            clk_div: ClkDiv::default(),
            sync_sel: SyncSel::default(),
            ..0i32.try_into().unwrap()
        }
    }
}

fn bool_to_i32(value: bool) -> i32 {
    if value {
        1
    } else {
        0
    }
}
