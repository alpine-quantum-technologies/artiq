use proc_macro2::TokenStream;
use quote::quote;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

mod ddb;
mod led;
mod ttl;
mod urukul;

/// Code generation for the kernel-support library.
///
/// - parse the generated device DB, extract the available peripherals
/// - emit code for the `PERIPHERALS` data in the `sinara` module.
pub fn generate_peripherals_code() {
    let ddb_path = generated_ddb_path();
    println!("cargo:rerun-if-changed={}", ddb_path.to_str().unwrap());

    let ddb_py = fs::read_to_string(&ddb_path).unwrap();
    let ddb = ddb_parser::parse(&ddb_py).unwrap();

    let core = ddb::core(&ddb).expect("Missing core device in device_db.");
    let code = vec![
        led::emit_code(&ddb),
        ttl::emit_code(&ddb),
        urukul::emit_code(&ddb, core),
    ];

    let definition_tokens = code.iter().map(|entry| &entry.definition_tokens);
    let instantiation_tokens = code.iter().map(|entry| &entry.instantiation_tokens);

    #[rustfmt::skip]
    let output = quote! {
	pub struct Peripherals {
	    #(#definition_tokens)*
	}

	pub const PERIPHERALS: Peripherals = Peripherals {
	    #(#instantiation_tokens)*
	};
    };

    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("peripherals.rs");
    fs::write(dest_path, formatted).unwrap();
}

/// Path to the device DB generated during the SoC build.
fn generated_ddb_path() -> PathBuf {
    let buildinc_dir = env::var("BUILDINC_DIRECTORY").unwrap();
    Path::new(&buildinc_dir)
        .join("generated")
        .join("device_db.py")
}

struct DeviceTypeCode {
    definition_tokens: TokenStream,
    instantiation_tokens: TokenStream,
}
