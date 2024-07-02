use crate::{core::Core, i2c};
use serde::Deserialize;
use serde_with::{serde_as, TryFromInto};

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    #[serde(rename = "artiq.coredevice.ad9910.AD9910")]
    Ad9910 { arguments: Ad9910 },
    #[serde(rename = "artiq.coredevice.phaser.Phaser")]
    Phaser { arguments: Phaser },
    #[serde(rename = "artiq.coredevice.spi2.SPIMaster")]
    Spi2Master { arguments: Spi2Master },
    #[serde(rename = "artiq.coredevice.kasli_i2c.KasliEEPROM")]
    KasliEeprom { arguments: KasliEeprom },

    #[serde(untagged)]
    Unknown(Ignored),
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct I2cSwitch {
    pub busno: Option<i32>,
    pub address: Option<u8>,
}

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
    pub acc_width: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct EdgeCounter {
    pub channel: i32,
    pub gateware_width: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct UrukulCpld {
    pub spi_device: String,
    pub io_update_device: Option<String>,
    pub dds_reset_device: Option<String>,
    pub sync_device: Option<String>,
    pub sync_sel: Option<sinara_config::urukul::SyncSel>,
    pub clk_sel: Option<sinara_config::urukul::ClkSel>,
    pub clk_div: Option<sinara_config::urukul::ClkDiv>,
    pub rf_sw: Option<u8>,
    pub refclk: Option<f64>,
    pub att: Option<u32>,
    pub sync_div: Option<u8>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Ad9910 {
    pub cpld_device: String,
    pub chip_select: i32,
    pub sw_device: Option<String>,
    pub pll_n: Option<i32>,
    pub pll_cp: Option<i32>,  // TODO: use ad9910-pac type
    pub pll_vco: Option<i32>, // TODO: use ad9910-pac type
    pub sync_delay_seed: Option<MaybeOnEeprom>,
    pub io_update_delay: Option<MaybeOnEeprom>,
    #[serde(default)]
    #[serde(deserialize_with = "optional_bool_from_int")]
    pub pll_en: Option<bool>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum MaybeOnEeprom {
    EepromAddress(EepromAddress),
    Value(u32),
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(try_from = "String")]
pub struct EepromAddress {
    pub eeprom_device: String,
    pub offset: u32,
}

impl TryFrom<String> for EepromAddress {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid EepromAddress format.".into());
        }

        Ok(Self {
            eeprom_device: parts[0].into(),
            offset: parts[1]
                .parse::<_>()
                .map_err(|e| format!("Invalid offset: {}", e))?,
        })
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Phaser {
    pub channel_base: i32,
    pub miso_delay: Option<i32>,
    pub tune_fifo_offset: Option<bool>,
    pub clk_sel: Option<u8>,
    pub sync_dly: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Spi2Master {
    pub channel: i32,
    pub div: Option<i32>,
    pub length: Option<i32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct KasliEeprom {
    #[serde_as(as = "TryFromInto<&str>")]
    pub port: i2c::KasliPort,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Ignored {}

/// Deserialize optional booleans from integers.
/// https://github.com/serde-rs/serde/issues/1344#issuecomment-410309140
fn optional_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<u8>::deserialize(deserializer)? {
        None => Ok(None),
        Some(0) => Ok(Some(false)),
        Some(1) => Ok(Some(true)),
        Some(other) => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_urukul_typical() {
        let data = r#"
            {
                "_qualclass": "artiq.coredevice.urukul.CPLD",
                "arguments": {
                    "spi_device": "spi_urukul0",
                    "sync_device": "ttl_urukul0_sync",
                    "io_update_device": "ttl_urukul0_io_update",
                    "refclk": 125000000.0,
                    "clk_sel": 2,
                    "clk_div": 0
                }
            }
        "#;

        let dev: Device = serde_json::from_str(data).unwrap();

        match dev {
            Device::UrukulCpld { arguments } => {
                assert_eq!(arguments.clk_sel, Some(sinara_config::urukul::ClkSel::Mmcx));
                assert_eq!(
                    arguments.clk_div,
                    Some(sinara_config::urukul::ClkDiv::Default)
                );
                assert!(arguments.sync_sel.is_none());
            }
            _ => panic!("Must be Urukul CPLD"),
        }
    }

    #[test]
    fn test_parse_ad9910_typical() {
        let data = r#"
            {
                "_qualclass": "artiq.coredevice.ad9910.AD9910",
                "arguments": {
                    "pll_en": 1,
                    "pll_n": 32,
                    "chip_select": 4,
                    "cpld_device": "urukul0_cpld",
                    "sw_device": "ttl_urukul0_sw0",
                    "sync_delay_seed": "eeprom_urukul0:64",
                    "io_update_delay": "eeprom_urukul0:68"
                }
           }
        "#;

        let dev: Device = serde_json::from_str(data).unwrap();

        match dev {
            Device::Ad9910 { arguments } => {
                assert!(arguments.pll_en.unwrap());
                assert_eq!(arguments.pll_n.unwrap(), 32);
                assert_eq!(arguments.chip_select, 4);
                assert_eq!(arguments.cpld_device, "urukul0_cpld");
                assert_eq!(
                    arguments.sync_delay_seed,
                    Some(MaybeOnEeprom::EepromAddress(EepromAddress {
                        eeprom_device: "eeprom_urukul0".into(),
                        offset: 64,
                    }))
                );
                assert_eq!(
                    arguments.io_update_delay,
                    Some(MaybeOnEeprom::EepromAddress(EepromAddress {
                        eeprom_device: "eeprom_urukul0".into(),
                        offset: 68,
                    }))
                );
            }
            _ => panic!("Must be Ad9910"),
        }
    }

    #[test]
    fn test_parse_ad9910_no_pll_en() {
        let data = r#"
            {
                "_qualclass": "artiq.coredevice.ad9910.AD9910",
                "arguments": {
                    "chip_select": 4,
                    "cpld_device": "urukul0_cpld"
                }
           }
        "#;

        let dev: Device = serde_json::from_str(data).unwrap();

        match dev {
            Device::Ad9910 { arguments } => {
                assert!(arguments.pll_en.is_none());
            }
            _ => panic!("Must be Ad9910."),
        }
    }

    #[test]
    fn test_parse_ad9910_pll_disabled() {
        let data = r#"
            {
                "_qualclass": "artiq.coredevice.ad9910.AD9910",
                "arguments": {
                    "pll_en": 0,
                    "chip_select": 4,
                    "cpld_device": "urukul0_cpld"
                }
           }
        "#;

        let dev: Device = serde_json::from_str(data).unwrap();

        match dev {
            Device::Ad9910 { arguments } => {
                assert_eq!(arguments.pll_en, Some(false));
            }
            _ => panic!("Must be Ad9910."),
        }
    }

    #[test]
    fn test_parse_ad9910_explicit_sync_delay_seed() {
        let data = r#"
            {
                "_qualclass": "artiq.coredevice.ad9910.AD9910",
                "arguments": {
                    "chip_select": 4,
                    "cpld_device": "urukul0_cpld",
                    "sync_delay_seed": 32
                }
           }
        "#;

        let dev: Device = serde_json::from_str(data).unwrap();

        match dev {
            Device::Ad9910 { arguments } => {
                assert_eq!(arguments.sync_delay_seed, Some(MaybeOnEeprom::Value(32)));
            }
            _ => panic!("Must be Ad9910."),
        }
    }

    #[test]
    fn test_parse_kasli_eeprom() {
        let data = r#"
            {
                "_qualclass": "artiq.coredevice.kasli_i2c.KasliEEPROM",
                "arguments": {
                    "port": "EEM3"
                }
            }
        "#;

        let dev: Device = serde_json::from_str(data).unwrap();

        match dev {
            Device::KasliEeprom { arguments } => {
                assert_eq!(arguments.port, i2c::KasliPort::Eem3)
            }
            _ => panic!("Must be KasliEEPROM."),
        }
    }
}
