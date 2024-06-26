extern crate ddb_parser;
extern crate itertools;
extern crate prettyplease;
extern crate proc_macro2;
extern crate quote;
extern crate regex;
extern crate syn;

use ddb_parser::{Device, DeviceDb};
use proc_macro2::TokenStream;
use regex::Regex;
use std::path::Path;
use std::{env, fs};

use itertools::Itertools;
use quote::quote;

/// Configuration and code generation for the kernel-support library.
///
/// - pass the configuration options set by the SoC definition.
/// - parse the generated device DB, extract the available peripherals.
/// - emit code for the `PERIPHERALS` data in the `sinara` module.
/// - set configuration options for the available channel types. These gate the
///   compilation of the corresponding syscalls.
fn main() {
    build_misoc::cfg();
    println!("cargo:rustc-cfg={}", "ksupport");

    let buildinc_dir = env::var("BUILDINC_DIRECTORY").unwrap();
    let ddb_path = Path::new(&buildinc_dir)
        .join("generated")
        .join("device_db.py");
    println!("cargo:rerun-if-changed={}", ddb_path.to_str().unwrap());

    let ddb_py = fs::read_to_string(&ddb_path).unwrap();
    let ddb = ddb_parser::parse(&ddb_py).unwrap();

    let code = vec![led_code(&ddb), ttl_out_code(&ddb)];

    let definition_tokens = code.iter().map(|entry| &entry.definition_tokens);
    let instantiation_tokens = code.iter().map(|entry| &entry.instantiation_tokens);

    #[rustfmt::skip]
    let output = quote! {
	pub struct Peripherals {
	    #(#definition_tokens),*
	}

	pub const PERIPHERALS: Peripherals = Peripherals {
            #(#instantiation_tokens),*
	};
    };

    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("peripherals.rs");
    fs::write(dest_path, formatted).unwrap();
}

/// `PERIPHERALS` data code for a specific device type.
struct DeviceTypeCode {
    definition_tokens: TokenStream,
    instantiation_tokens: TokenStream,
}

/// Emit code for the `PERIPHERALS` data for standalone `TTLOut` devices.
///
/// Standalone `TTLOut` devices have names matching `ttl[0-9]+`.
/// This excludes in particular LEDs and Urukul-linked signaling lines.
fn ttl_out_code(ddb: &DeviceDb) -> DeviceTypeCode {
    let key_regex = Regex::new(r"^ttl\d+$").unwrap();

    let channels: Vec<_> = ddb
        .iter()
        .filter_map(|entry| match entry {
            (key, Device::TtlOut { arguments }) => {
                if key_regex.is_match(key) {
                    Some(arguments.channel)
                } else {
                    None
                }
            }
            _ => None,
        })
        .sorted()
        .collect();

    let count = channels.len();
    if count > 0 {
        println!("cargo:rustc-cfg={}", "has_sinara_ttl_out");
    }

    DeviceTypeCode {
        definition_tokens: quote! {
            ttl_out: [ttl::TtlOut; #count]
        },
        instantiation_tokens: quote! {
            ttl_out: [#( ttl::TtlOut { channel: #channels } ),*]
        },
    }
}

/// Emit code for the `PERIPHERALS` data for LED devices.
fn led_code(ddb: &DeviceDb) -> DeviceTypeCode {
    let key_regex = Regex::new(r"^led\d+$").unwrap();

    let channels: Vec<_> = ddb
        .iter()
        .filter_map(|entry| match entry {
            (key, Device::TtlOut { arguments }) => {
                if key_regex.is_match(key) {
                    Some(arguments.channel)
                } else {
                    None
                }
            }
            _ => None,
        })
        .sorted()
        .collect();

    let count = channels.len();
    if count > 0 {
        println!("cargo:rustc-cfg={}", "has_sinara_led");
    }

    DeviceTypeCode {
        definition_tokens: quote! {
            led: [ttl::TtlOut; #count]
        },
        instantiation_tokens: quote! {
            led: [#( ttl::TtlOut { channel: #channels } ),*]
        },
    }
}
