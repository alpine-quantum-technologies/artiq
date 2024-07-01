use crate::DeviceTypeCode;
use ddb_parser::{Device, DeviceDb};
use itertools::Itertools;
use lazy_static::lazy_static;
use quote::quote;
use regex::Regex;

pub(crate) fn emit_code(ddb: &DeviceDb) -> DeviceTypeCode {
    lazy_static! {
        static ref KEY_REGEX: Regex = Regex::new(r"^led\d+$").unwrap();
    }

    let channels: Vec<_> = ddb
        .iter()
        .filter_map(|entry| match entry {
            (key, Device::TtlOut { arguments }) if KEY_REGEX.is_match(key) => {
                Some(arguments.channel)
            }
            _ => None,
        })
        .sorted()
        .collect();

    let count = channels.len();
    if count > 0 {
        println!("cargo:rustc-cfg=has_sinara_led");
    }

    DeviceTypeCode {
        definition_tokens: quote! {
            led: [ttl::TtlOut; #count],
        },
        instantiation_tokens: quote! {
            led: [#(ttl::TtlOut { channel: #channels }),*],
        },
    }
}
