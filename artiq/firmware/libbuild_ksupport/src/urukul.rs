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

    /// I²C multiplexers
    i2c_switches: (&'a ddb_parser::i2c::Switch, &'a ddb_parser::i2c::Switch),
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

    /// Calibrated synchronization data source.
    sync_data_source: sinara_config::urukul::SyncDataSource,
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

                let sync_data_source = {
                    use ddb_parser::eeprom::MaybeOnEeprom;

                    match (&ch.sync_delay_seed, &ch.io_update_delay) {
                        (
                            MaybeOnEeprom::Value(sync_delay_seed),
                            MaybeOnEeprom::Value(io_update_delay),
                        ) => sinara_config::urukul::SyncDataSource::User {
                            sync_delay_seed: *sync_delay_seed as u8,
                            io_update_delay: *io_update_delay as u8,
			    validation_window: 0, // FIXME: expand device_db format
                        },
			(
			    MaybeOnEeprom::EepromAddress(sync_delay_seed_loc),
			    MaybeOnEeprom::EepromAddress(io_update_delay_loc),
			) => {
			    if sync_delay_seed_loc != io_update_delay_loc {
				panic!("When reading from EEPROM, sync_delay_seed must be the same as io_update_delay ({})", key);
			    }

			    let eeprom = ddb::eeprom(&sync_delay_seed_loc.eeprom_device, ddb)
				.unwrap_or_else(|| panic!("Missing EEPROM device {} for {}", sync_delay_seed_loc.eeprom_device, key));

			    sinara_config::urukul::SyncDataSource::Eeprom {
				port: eeprom.port,
				offset: sync_delay_seed_loc.offset,
			    }
			},
                        _ => unimplemented!(),
                    }
                };

                Some(Channel {
                    switch_device,
                    pll_en: ch.pll_en,
                    pll_n: ch.pll_n as u8,
                    pll_vco: ch.pll_vco,
                    pll_cp: ch.pll_cp,
                    sync_data_source,
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Channel mismatch for {}", key));

        let i2c_switches = ddb::i2c_switches(ddb).expect("Missing I²C switches.");

        Self {
            spi_channel: spi_device.channel,
            clk_sel: ClkSel(dev.clk_sel),
            clk_div: ClkDiv(dev.clk_div),
            sync_div,
            sync_device,
            io_update_device,
            core,
            channels,
            i2c_switches,
        }
    }

    fn tokens(&self) -> TokenStream {
        let bus_tokens = self.bus_tokens();
        let i2c_bus_tokens = self.i2c_bus_tokens();
        let config_tokens = self.config_tokens();
        let sync_tokens = self.sync_tokens();
        let io_update_tokens = self.io_update_tokens();
        let channels_tokens = self.channels_tokens();

        #[rustfmt::skip]
        quote! {
            urukul::Cpld {
		#bus_tokens,
		#i2c_bus_tokens,
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

    fn i2c_bus_tokens(&self) -> TokenStream {
        let switch0_address = self.i2c_switches.0.address as i32;
        let switch1_address = self.i2c_switches.1.address as i32;

        #[rustfmt::skip]
	quote! {
	    i2c_bus: i2c::KasliI2C {
		switch0: i2c::Switch {
		    busno: 0,
		    address: #switch0_address,
		},
		switch1: i2c::Switch {
		    busno: 0,
		    address: #switch1_address,
		},
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

            // TODO: implement ToTokens for SyncDataSource in libconfig_sinara?
            let sync_data_source = match ch.sync_data_source {
                sinara_config::urukul::SyncDataSource::User {
                    sync_delay_seed,
                    io_update_delay,
                    validation_window,
                } => {
                    #[rustfmt::skip]
		    quote! {
			sinara_config::urukul::SyncDataSource::User {
			    sync_delay_seed: #sync_delay_seed,
			    io_update_delay: #io_update_delay,
			    validation_window: #validation_window,
			}
		    }
                }
                sinara_config::urukul::SyncDataSource::Eeprom { port, offset } => {
                    let port_ident = debug_ident(port);

                    #[rustfmt::skip]
		    quote! {
			sinara_config::urukul::SyncDataSource::Eeprom {
			    port: sinara_config::i2c::KasliPort::#port_ident,
			    offset: #offset,
			}
		    }
                }
            };

            #[rustfmt::skip]
            quote! {
		urukul::ChannelDesc {
                    switch_device: ttl::TtlOut { channel: #sw_channel },
		    pll_cp: ad9910_pac::cfr3::ICpA::#pll_cp,
		    pll_vco: ad9910_pac::cfr3::VcoSelA::#pll_vco,
		    pll_n: #pll_n,
		    pll_en: #pll_en,
		    sync_data_source: #sync_data_source,
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
