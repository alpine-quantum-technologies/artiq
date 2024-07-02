use crate::{core::Core, eeprom, i2c, phaser, spi2, ttl, urukul};
use serde::Deserialize;

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
    I2cSwitch { arguments: i2c::Switch },
    #[serde(rename = "artiq.coredevice.ttl.TTLOut")]
    TtlOut { arguments: ttl::TtlOut },
    #[serde(rename = "artiq.coredevice.ttl.TTLInOut")]
    TtlInOut { arguments: ttl::TtlInOut },
    #[serde(rename = "artiq.coredevice.ttl.TTLClockGen")]
    TtlClockGen { arguments: ttl::TtlClockGen },
    #[serde(rename = "artiq.coredevice.edge_counter.EdgeCounter")]
    EdgeCounter { arguments: ttl::EdgeCounter },
    #[serde(rename = "artiq.coredevice.urukul.CPLD")]
    UrukulCpld { arguments: urukul::Cpld },
    #[serde(rename = "artiq.coredevice.ad9910.AD9910")]
    Ad9910 { arguments: urukul::Ad9910 },
    #[serde(rename = "artiq.coredevice.phaser.Phaser")]
    Phaser { arguments: phaser::Phaser },
    #[serde(rename = "artiq.coredevice.spi2.SPIMaster")]
    Spi2Master { arguments: spi2::Master },
    #[serde(rename = "artiq.coredevice.kasli_i2c.KasliEEPROM")]
    KasliEeprom { arguments: eeprom::Kasli },

    #[serde(untagged)]
    Unknown(Ignored),
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Ignored {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eeprom::{EepromAddress, MaybeOnEeprom};

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
                assert_eq!(arguments.clk_sel, sinara_config::urukul::ClkSel::Mmcx);
                assert_eq!(arguments.clk_div, sinara_config::urukul::ClkDiv::Default);
                assert_eq!(
                    arguments.sync_sel,
                    sinara_config::urukul::SyncSel::default()
                );
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
                assert!(arguments.pll_en);
                assert_eq!(arguments.pll_n, 32);
                assert_eq!(arguments.chip_select, 4);
                assert_eq!(arguments.cpld_device, "urukul0_cpld");
                assert_eq!(
                    arguments.sync_delay_seed,
                    MaybeOnEeprom::EepromAddress(EepromAddress {
                        eeprom_device: "eeprom_urukul0".into(),
                        offset: 64,
                    })
                );
                assert_eq!(
                    arguments.io_update_delay,
                    MaybeOnEeprom::EepromAddress(EepromAddress {
                        eeprom_device: "eeprom_urukul0".into(),
                        offset: 68,
                    })
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
                assert!(arguments.pll_en);
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
                assert!(!arguments.pll_en);
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
                assert_eq!(arguments.sync_delay_seed, MaybeOnEeprom::Value(32));
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
                assert_eq!(arguments.port, sinara_config::i2c::KasliPort::Eem3)
            }
            _ => panic!("Must be KasliEEPROM."),
        }
    }
}
