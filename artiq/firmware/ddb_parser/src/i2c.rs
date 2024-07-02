use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Switch {
    #[serde(default = "default_busno")]
    pub busno: i32,

    #[serde(default = "default_address")]
    pub address: u8,
}

fn default_busno() -> i32 {
    0
}

fn default_address() -> u8 {
    0xe8
}
