use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Master {
    pub channel: i32,
    #[serde(default)]
    pub div: i32,
    #[serde(default)]
    pub length: i32,
}
