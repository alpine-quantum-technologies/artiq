use crate::DeviceTypeCode;
use ddb_parser::{Device, DeviceDb};
use itertools::Itertools;
use lazy_static::lazy_static;
use quote::quote;
use regex::Regex;

pub(crate) fn emit_code(ddb: &DeviceDb) -> DeviceTypeCode {
    let ttl_out = emit_ttl_out_code(ddb);
    let ttl_out_def = ttl_out.definition_tokens;
    let ttl_out_inst = ttl_out.instantiation_tokens;

    DeviceTypeCode {
        definition_tokens: quote! {
            #ttl_out_def
        },
        instantiation_tokens: quote! {
            #ttl_out_inst
        },
    }
}

fn emit_ttl_out_code(ddb: &DeviceDb) -> DeviceTypeCode {
    lazy_static! {
        static ref KEY_REGEX: Regex = Regex::new(r"^ttl\d+$").unwrap();
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
        println!("cargo:rustc-cfg=has_sinara_ttl_out");
    }

    DeviceTypeCode {
        definition_tokens: quote! {
            ttl_out: [ttl::TtlOut; #count],
        },
        instantiation_tokens: quote! {
            ttl_out: [#(ttl::TtlOut { channel: #channels }),*],
        },
    }
}
