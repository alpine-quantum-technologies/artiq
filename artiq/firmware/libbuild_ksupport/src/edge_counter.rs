use crate::DeviceTypeCode;
use ddb_parser::{Device, DeviceDb};
use itertools::Itertools;
use quote::quote;

pub(crate) fn emit_code(ddb: &DeviceDb) -> DeviceTypeCode {
    let args: Vec<_> = ddb
        .iter()
        .filter_map(|entry| match entry.1 {
            Device::EdgeCounter { arguments } => Some(arguments),
            _ => None,
        })
        .sorted_by_key(|args| args.channel)
        .collect();

    let count = args.len();
    if count > 0 {
        println!("cargo:rustc-cfg=has_sinara_edge_counter");
    }

    let channels = args.iter().map(|entry| entry.channel);
    let gateware_widths = args.iter().map(|entry| entry.gateware_width);

    DeviceTypeCode {
        definition_tokens: quote! {
            edge_counter:  [edge_counter::EdgeCounter; #count],
        },
        instantiation_tokens: quote! {
            edge_counter: [#(edge_counter::EdgeCounter {
            channel: #channels,
            gateware_width: #gateware_widths,
            }),*],
        },
    }
}
