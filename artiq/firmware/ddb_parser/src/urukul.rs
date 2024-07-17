use crate::eeprom::MaybeOnEeprom;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Cpld {
    pub spi_device: String,
    pub io_update_device: Option<String>,
    pub dds_reset_device: Option<String>,
    pub sync_device: Option<String>,
    #[serde(default)]
    pub sync_sel: sinara_config::urukul::SyncSel,
    #[serde(default)]
    pub clk_sel: sinara_config::urukul::ClkSel,
    #[serde(default)]
    pub clk_div: sinara_config::urukul::ClkDiv,
    #[serde(default)]
    pub rf_sw: u8,
    #[serde(default = "default_refclk")]
    pub refclk: f64,
    #[serde(default)]
    pub att: u32,
    pub sync_div: Option<u8>,
}

fn default_refclk() -> f64 {
    125e6
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Ad9910 {
    pub cpld_device: String,
    pub chip_select: i32,
    pub sw_device: Option<String>,
    #[serde(default = "default_pll_n")]
    pub pll_n: i32,
    #[serde(default)]
    pub pll_cp: ad9910_pac::cfr3::ICpA,
    #[serde(default = "default_pll_vco")]
    pub pll_vco: ad9910_pac::cfr3::VcoSelA,
    #[serde(default)]
    pub sync_delay_seed: MaybeOnEeprom,
    #[serde(default)]
    pub io_update_delay: MaybeOnEeprom,
    #[serde(default = "default_pll_en")]
    #[serde(deserialize_with = "bool_from_int")]
    pub pll_en: bool,
}

fn default_pll_n() -> i32 {
    40
}

fn default_pll_vco() -> ad9910_pac::cfr3::VcoSelA {
    ad9910_pac::cfr3::VcoSelA::Vco5
}

fn default_pll_en() -> bool {
    true
}

/// Deserialize booleans from integers.
/// https://github.com/serde-rs/serde/issues/1344#issuecomment-410309140
fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
