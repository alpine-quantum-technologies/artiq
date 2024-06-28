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

    let core = find_core(&ddb).expect("Missing core device.");
    let code = vec![led_code(&ddb), ttl_out_code(&ddb), urukul_code(&ddb, &core)];

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

/// Emit code for the `PERIPHERALS` data for Urukul devices.
fn urukul_code(ddb: &DeviceDb, core: &ddb_parser::devices::Core) -> DeviceTypeCode {
    let urukuls: Vec<_> = ddb
        .iter()
        .filter_map(|entry| match entry {
            (key, Device::UrukulCpld { arguments }) => {
                Some(Urukul::from_ddb(key, arguments, ddb, core))
            }
            _ => None,
        })
        .sorted_by_key(|entry| entry.spi_channel)
        .collect();

    let count = urukuls.len();
    if count > 0 {
        println!("cargo:rustc-cfg={}", "has_sinara_urukul");
    }

    let tokens = urukuls.iter().map(|entry| entry.tokens());

    DeviceTypeCode {
        definition_tokens: quote! {
            urukul: [urukul::Cpld; #count]
        },
        instantiation_tokens: quote! {
            urukul: [#( #tokens ),*]
        },
    }
}

/// Search a SPI master device by key.
fn find_spi_device<'a, 'b>(
    key: &'a str,
    ddb: &'b DeviceDb,
) -> Option<&'b ddb_parser::devices::Spi2Master> {
    for entry in ddb {
        match entry {
            (ddb_key, Device::Spi2Master { arguments }) if key == ddb_key => {
                return Some(arguments);
            }
            _ => continue,
        }
    }

    None
}

/// Search the core device in the device DB.
fn find_core(ddb: &DeviceDb) -> Option<&ddb_parser::devices::Core> {
    for entry in ddb {
        if let Device::Core { arguments } = entry.1 {
            return Some(arguments);
        }
    }

    None
}

/// Helper for Urukul code generation.
#[derive(Debug)]
struct Urukul<'a> {
    /// RTIO channel that controls the SPI master.
    spi_channel: i32,

    /// Clock selection.
    clk_sel: u8,

    /// Clock divider.
    clk_div: u8,

    /// Synchronization clock divider.
    #[allow(dead_code)]
    sync_div: u8,

    /// Synchronization source selection.
    sync_sel: u8,

    /// Core device.
    core: &'a ddb_parser::devices::Core,
}

impl<'a> Urukul<'a> {
    /// Fill a `Urukul` struct from a particular device DB entry.
    ///
    /// # Arguments
    ///
    /// * `key` - device key
    /// * `dev` - entry content for `key`
    /// * `ddb` - the containing device DB
    /// * `core` - the containing device DB's core device.
    fn from_ddb(
        key: &str,
        dev: &ddb_parser::devices::UrukulCpld,
        ddb: &DeviceDb,
        core: &'a ddb_parser::devices::Core,
    ) -> Self {
        let (sync_sel, sync_div) = if dev.sync_device.is_some() {
            (0, dev.sync_div.unwrap_or(2))
        } else {
            (1, dev.sync_div.unwrap_or(0))
        };

        Self {
            spi_channel: find_spi_device(&dev.spi_device, ddb)
                .expect(format!("Missing SPI device for Urukul {}", key).as_str())
                .channel,
            clk_sel: dev.clk_sel.unwrap_or(0),
            clk_div: dev.clk_div.unwrap_or(0),
            sync_div,
            sync_sel,
            core,
        }
    }

    /// Generated code for the Urukul device.
    fn tokens(&self) -> TokenStream {
        let bus_tokens = self.bus_tokens();
        let config_tokens = self.config_tokens();

        quote! {
            urukul::Cpld { #bus_tokens, #config_tokens }
        }
    }

    /// Generated code for the SPI bus driver.
    fn bus_tokens(&self) -> TokenStream {
        let channel = self.spi_channel;
        let ref_period_mu = self.core.ref_multiplier.unwrap_or(8) as i64;

        #[rustfmt::skip]
        quote! {
            bus: spi2::Bus {
		channel: #channel,
		ref_period_mu: #ref_period_mu,
            }
        }
    }

    /// Generated code for the CPLD configuration.
    fn config_tokens(&self) -> TokenStream {
        // FIXME: use actual type for the conversion
        let clk_sel = match self.clk_sel {
            0 => quote! { urukul::ClkSel::Internal },
            1 => quote! { urukul::ClkSel::Sma },
            2 => quote! { urukul::ClkSel::Mmcx },
            _ => panic!("Invalid clk_sel"),
        };

        let clk_div = match self.clk_div {
            0 => quote! { urukul::ClkDiv::Default },
            1 => quote! { urukul::ClkDiv::One },
            2 => quote! { urukul::ClkDiv::Two },
            3 => quote! { urukul::ClkDiv::Four },
            _ => panic!("Invalid clk_div"),
        };

        let sync_sel = match self.sync_sel {
            0 => quote! { urukul::SyncSel::Eem },
            1 => quote! { urukul::SyncSel::Dds0 },
            _ => panic!("Invalid sync_sel"),
        };

        #[rustfmt::skip]
	quote! {
	    config: urukul::Config {
		profile: 7,
		io_update: false,
		reset: false,
		io_reset: false,
		clk_sel: #clk_sel,
		clk_div: #clk_div,
		sync_sel: #sync_sel,
	    }
	}
    }
}
