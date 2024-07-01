use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Phaser {
    pub channel_base: i32,
    #[serde(default = "default_miso_delay")]
    pub miso_delay: i32,
    #[serde(default = "default_tune_fifo_offset")]
    pub tune_fifo_offset: bool,
    #[serde(default)]
    pub clk_sel: sinara_config::phaser::ClkSel,
    #[serde(default)]
    pub sync_dly: i32,
}

fn default_miso_delay() -> i32 {
    1
}

fn default_tune_fifo_offset() -> bool {
    true
}
