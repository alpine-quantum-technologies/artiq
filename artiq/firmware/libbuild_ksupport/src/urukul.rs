use crate::{ddb, DeviceTypeCode};
use ddb_parser::{Device, DeviceDb};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::fmt::Debug;

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

#[derive(Debug)]
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

    /// I/O update device.
    io_update_device: Option<&'a ddb_parser::ttl::TtlOut>,

    /// Core device.
    core: &'a ddb_parser::core::Core,

    /// RF channels
    channels: [Channel<'a>; 4],
}

#[derive(Debug)]
struct Channel<'a> {
    /// Switch TTL.
    switch_device: &'a ddb_parser::ttl::TtlOut,

    /// Whether to enable the reference clock PLL.
    pll_en: bool,

    /// Reference clock PLL N divider.
    pll_n: u8,

    /// Reference clock PLL VCO selection.
    pll_vco: ad9910_pac::cfr3::VcoSelA,

    /// Reference clock PLL charge pump current.
    pll_cp: ad9910_pac::cfr3::ICpA,
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
                .unwrap_or_else(|| panic!("Missing sync generator for {}", key));

            (Some(sync_device), dev.sync_div.unwrap_or(2))
        } else {
            (None, dev.sync_div.unwrap_or(0))
        };

        let spi_device = ddb::spi_device(&dev.spi_device, ddb)
            .unwrap_or_else(|| panic!("Missing SPI device for {}", key));

        let io_update_device = dev
            .io_update_device
            .as_ref()
            .and_then(|key| ddb::ttl_out(key, ddb));

        let channels: [_; 4] = ddb::urukul_channels(key, ddb)
            .filter_map(|ch| {
                let switch_device = ch
                    .sw_device
                    .as_ref()
                    .map(|sw_dev_key| ddb::ttl_out(sw_dev_key, ddb))
                    .flatten()?;

                Some(Channel {
                    switch_device,
                    pll_en: ch.pll_en,
                    pll_n: ch.pll_n as u8,
                    pll_vco: ch.pll_vco,
                    pll_cp: ch.pll_cp,
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Channel mismatch for {}", key));

        Self {
            spi_channel: spi_device.channel,
            clk_sel: ClkSel(dev.clk_sel),
            clk_div: ClkDiv(dev.clk_div),
            sync_div,
            sync_device,
            io_update_device,
            core,
            channels,
        }
    }

    fn tokens(&self) -> TokenStream {
        let bus_tokens = self.bus_tokens();
        let config_tokens = self.config_tokens();
        let sync_tokens = self.sync_tokens();
        let io_update_tokens = self.io_update_tokens();
        let channels_tokens = self.channels_tokens();

        #[rustfmt::skip]
        quote! {
            urukul::Cpld {
		#bus_tokens,
		#config_tokens,
		#sync_tokens,
		#io_update_tokens,
		#channels_tokens,
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

    fn io_update_tokens(&self) -> TokenStream {
        let def = if let Some(io_update_device) = self.io_update_device {
            let channel = io_update_device.channel;

            #[rustfmt::skip]
	    quote! {
		Some(ttl::TtlOut { channel: #channel })
	    }
        } else {
            #[rustfmt::skip]
            quote! {
		None
            }
        };

        quote! {
            io_update: #def
        }
    }

    fn channels_tokens(&self) -> TokenStream {
        let channel_descs = self.channels.iter().map(|ch| {
            let sw_channel = ch.switch_device.channel;
            let pll_cp = debug_ident(ch.pll_cp);
            let pll_vco = debug_ident(ch.pll_vco);
            let pll_n = ch.pll_n;
            let pll_en = ch.pll_en;

            #[rustfmt::skip]
            quote! {
		urukul::ChannelDesc {
                    switch_device: ttl::TtlOut { channel: #sw_channel },
		    pll_cp: ad9910_pac::cfr3::ICpA::#pll_cp,
		    pll_vco: ad9910_pac::cfr3::VcoSelA::#pll_vco,
		    pll_n: #pll_n,
		    pll_en: #pll_en,
		}
            }
        });

        quote! {
            channels: [#(#channel_descs),*]
        }
    }
}

#[derive(Debug)]
struct ClkSel(sinara_config::urukul::ClkSel);

impl ToTokens for ClkSel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = debug_ident(self.0);
        tokens.append_all(quote! { sinara_config::urukul::ClkSel::#variant });
    }
}

#[derive(Debug)]
struct ClkDiv(sinara_config::urukul::ClkDiv);

impl ToTokens for ClkDiv {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = debug_ident(self.0);
        tokens.append_all(quote! { sinara_config::urukul::ClkDiv::#variant });
    }
}

#[derive(Debug)]
struct SyncSel(sinara_config::urukul::SyncSel);

impl ToTokens for SyncSel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = debug_ident(self.0);
        tokens.append_all(quote! { sinara_config::urukul::SyncSel::#variant });
    }
}

fn debug_ident<T: Debug>(val: T) -> proc_macro2::Ident {
    format_ident!("{}", format!("{:?}", val))
}
