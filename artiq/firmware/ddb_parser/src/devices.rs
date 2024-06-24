use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "_qualclass")]
pub enum Device {
    #[serde(rename = "artiq.coredevice.core.Core")]
    Core { arguments: Core },
    #[serde(rename = "artiq.coredevice.cache.CoreCache")]
    CoreCache {},
    #[serde(rename = "artiq.coredevice.dma.CoreDMA")]
    CoreDma {},
    #[serde(rename = "artiq.coredevice.i2c.I2CSwitch")]
    I2cSwitch { arguments: I2cSwitch },
    #[serde(rename = "artiq.coredevice.ttl.TTLOut")]
    TtlOut { arguments: TtlOut },
    #[serde(rename = "artiq.coredevice.ttl.TTLInOut")]
    TtlInOut { arguments: TtlInOut },
    #[serde(rename = "artiq.coredevice.ttl.TTLClockGen")]
    TtlClockGen { arguments: TtlClockGen },
    #[serde(rename = "artiq.coredevice.edge_counter.EdgeCounter")]
    EdgeCounter { arguments: EdgeCounter },
    #[serde(rename = "artiq.coredevice.urukul.CPLD")]
    UrukulCpld { arguments: UrukulCpld },
    #[serde(rename = "artiq.coredevice.phaser.Phaser")]
    Phaser { arguments: Phaser },

    #[serde(untagged)]
    Unknown(Ignored),
}

#[derive(Debug, Deserialize)]
pub struct Core {
    pub host: String,
    pub ref_period: f32,
    pub ref_multiplier: Option<i32>,
    pub target: Option<Target>,
}

#[derive(Debug, Deserialize)]
pub enum Target {
    #[serde(rename = "rv32ima")]
    Kasli1,
    #[serde(rename = "rv32g")]
    Kasli2,
    #[serde(rename = "cortexa9")]
    KasliSoc,
}

#[derive(Debug, Deserialize)]
pub struct I2cSwitch {
    pub busno: Option<i32>,
    pub address: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct TtlOut {
    pub channel: i32,
}

#[derive(Debug, Deserialize)]
pub struct TtlInOut {
    pub channel: i32,
    pub gate_latency_mu: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct TtlClockGen {
    pub channel: i32,
    pub acc_width: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EdgeCounter {
    pub channel: i32,
    pub gateware_width: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UrukulCpld {
    pub spi_device: String,
    pub io_update_device: Option<String>,
    pub dds_reset_device: Option<String>,
    pub sync_device: Option<String>,
    pub sync_sel: Option<u8>,
    pub clk_sel: Option<u8>,
    pub clk_div: Option<u8>,
    pub rf_sw: Option<u8>,
    pub refclk: Option<f64>,
    pub att: Option<u32>,
    pub sync_div: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Phaser {
    pub channel_base: i32,
    pub miso_delay: Option<i32>,
    pub tune_fifo_offset: Option<bool>,
    pub clk_sel: Option<u8>,
    pub sync_dly: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Ignored {}
