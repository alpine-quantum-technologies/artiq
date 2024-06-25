extern crate ddb_parser;
extern crate itertools;
extern crate prettyplease;
extern crate quote;
extern crate syn;

use ddb_parser::Device;
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

    let ttl_out_channels: Vec<_> = ddb
        .iter()
        .filter_map(|elem| match elem {
            (key, Device::TtlOut { arguments }) => {
                if key.contains("ttl") {
                    Some(arguments.channel)
                } else {
                    None
                }
            }
            _ => None,
        })
        .sorted()
        .collect();

    let num_ttl_out_channels = ttl_out_channels.len();
    if num_ttl_out_channels > 0 {
        println!("cargo:rustc-cfg={}", "has_sinara_ttl_out");
    }

    #[rustfmt::skip]
    let output = quote! {
	pub struct Peripherals {
            ttl_out: [ttl::TtlOut; #num_ttl_out_channels],
	}

	pub const PERIPHERALS: Peripherals = Peripherals {
            ttl_out: [#( ttl::TtlOut { channel: #ttl_out_channels } ),*],
	};
    };

    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("peripherals.rs");
    fs::write(dest_path, formatted).unwrap();
}
