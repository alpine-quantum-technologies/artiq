use crate::{ddb, DeviceTypeCode};
use ddb_parser::{Device, DeviceDb};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

pub(crate) fn emit_code(ddb: &DeviceDb, core: &ddb_parser::core::Core) -> DeviceTypeCode {
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
        println!("cargo:rustc-cfg=has_sinara_urukul");
    }

    let tokens = urukuls.iter().map(|entry| entry.tokens());

    DeviceTypeCode {
        definition_tokens: quote! {
            urukul: [urukul::Cpld; #count],
        },
        instantiation_tokens: quote! {
            urukul: [#(#tokens),*],
        },
    }
}

struct Urukul<'a> {
    /// RTIO channel that control the SPI master.
    spi_channel: i32,

    /// Clock selection.
    clk_sel: ClkSel,

    /// Clock divider.
    clk_div: ClkDiv,

    /// Synchronization clock divider.
    sync_div: u8,

    /// Synchronization clock generator.
    sync_device: Option<&'a ddb_parser::ttl::TtlClockGen>,

    /// Core device.
    core: &'a ddb_parser::core::Core,
}

impl<'a> Urukul<'a> {
    /// Fill a `Urukul` struct from a particular device DB entry.
    ///
    /// # Arguments
    ///
    /// - `key` - device key
    /// - `dev` - entry content for `key`
    /// - `ddb` - the containing device DB
    /// - `core` - the containing device DB's core device.
    fn from_ddb(
        key: &str,
        dev: &'a ddb_parser::urukul::Cpld,
        ddb: &'a DeviceDb,
        core: &'a ddb_parser::core::Core,
    ) -> Self {
        let (sync_device, sync_div) = if let Some(sync_device_key) = &dev.sync_device {
            let sync_device = ddb::ttl_clock_gen(sync_device_key, ddb)
                .unwrap_or_else(|| panic!("Missing sync generator for Urukul {}", key));

            (Some(sync_device), dev.sync_div.unwrap_or(2))
        } else {
            (None, dev.sync_div.unwrap_or(0))
        };

        let spi_device = ddb::spi_device(&dev.spi_device, ddb)
            .unwrap_or_else(|| panic!("Missing SPI device for Urukul {}", key));

        Self {
            spi_channel: spi_device.channel,
            clk_sel: ClkSel(dev.clk_sel),
            clk_div: ClkDiv(dev.clk_div),
            sync_div,
            sync_device,
            core,
        }
    }

    fn tokens(&self) -> TokenStream {
        let bus_tokens = self.bus_tokens();
        let config_tokens = self.config_tokens();
        let sync_tokens = self.sync_tokens();

        #[rustfmt::skip]
        quote! {
            urukul::Cpld {
		#bus_tokens,
		#config_tokens,
		#sync_tokens,
            }
        }
    }

    fn bus_tokens(&self) -> TokenStream {
        let channel = self.spi_channel;
        let ref_period_mu = self.core.ref_multiplier as i64;

        #[rustfmt::skip]
	quote! {
	    bus: spi2::Bus {
		channel: #channel,
		ref_period_mu: #ref_period_mu,
	    }
	}
    }

    fn config_tokens(&self) -> TokenStream {
        let clk_sel = &self.clk_sel;
        let clk_div = &self.clk_div;
        let sync_sel = if self.sync_device.is_some() {
            SyncSel(sinara_config::urukul::SyncSel::Eem)
        } else {
            SyncSel(sinara_config::urukul::SyncSel::Dds0)
        };

        #[rustfmt::skip]
	quote! {
	    config: sinara_config::urukul::Config {
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

    fn sync_tokens(&self) -> TokenStream {
        let def = if let Some(sync_device) = &self.sync_device {
            println!("cargo:rustc-cfg=has_sinara_ttl_clk_gen");

            let channel = sync_device.channel;
            let acc_width = sync_device.acc_width as i64;
            let sync_div = &self.sync_div;

            #[rustfmt::skip]
            quote! {
		Some(urukul::SyncGen {
                    device: ttl::TtlClockGen {
			channel: #channel,
			acc_width: #acc_width,
                    },
                    div: #sync_div,
		})
            }
        } else {
            #[rustfmt::skip]
            quote! {
		None
            }
        };

        quote! {
            sync: #def
        }
    }
}

struct ClkSel(sinara_config::urukul::ClkSel);

impl ToTokens for ClkSel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = format_ident!("{}", format!("{:?}", self.0));
        tokens.append_all(quote! { sinara_config::urukul::ClkSel::#variant });
    }
}

struct ClkDiv(sinara_config::urukul::ClkDiv);

impl ToTokens for ClkDiv {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = format_ident!("{}", format!("{:?}", self.0));
        tokens.append_all(quote! { sinara_config::urukul::ClkDiv::#variant });
    }
}

struct SyncSel(sinara_config::urukul::SyncSel);

impl ToTokens for SyncSel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = format_ident!("{}", format!("{:?}", self.0));
        tokens.append_all(quote! { sinara_config::urukul::SyncSel::#variant });
    }
}
