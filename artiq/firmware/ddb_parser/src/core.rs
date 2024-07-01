use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Core {
    pub host: String,
    pub ref_period: f32,

    #[serde(default = "default_ref_multiplier")]
    pub ref_multiplier: u32,

    #[serde(default)]
    pub target: Target,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum Target {
    #[serde(rename = "rv32ima")]
    Kasli1,
    #[serde(rename = "rv32g")]
    Kasli2,
    #[serde(rename = "cortexa9")]
    KasliSoc,
}

impl Default for Target {
    fn default() -> Self {
        Self::Kasli2
    }
}

fn default_ref_multiplier() -> u32 {
    8
}
