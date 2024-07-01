use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct TtlOut {
    pub channel: i32,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct TtlInOut {
    pub channel: i32,
    pub gate_latency_mu: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct TtlClockGen {
    pub channel: i32,

    #[serde(default = "default_clk_gen_acc_width")]
    pub acc_width: i32,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct EdgeCounter {
    pub channel: i32,

    #[serde(default = "default_edge_counter_gateware_width")]
    pub gateware_width: i32,
}

fn default_clk_gen_acc_width() -> i32 {
    24
}

fn default_edge_counter_gateware_width() -> i32 {
    31
}
