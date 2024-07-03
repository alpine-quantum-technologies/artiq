use ddb_parser::{Device, DeviceDb};
use itertools::Itertools;

/// First matching SPI master device (if any).
///
/// # Arguments
/// - `key` - target device key.
/// - `ddb` - device DB to search in.
pub(crate) fn spi_device<'a, 'b>(
    key: &'a str,
    ddb: &'b DeviceDb,
) -> Option<&'b ddb_parser::spi2::Master> {
    for entry in ddb {
        match entry {
            (ddb_key, Device::Spi2Master { arguments }) if key == ddb_key => {
                return Some(arguments)
            }
            _ => continue,
        }
    }
    None
}

/// First core device (if any).
///
/// # Arguments
/// - `ddb` - device DB to search in.
pub(crate) fn core(ddb: &DeviceDb) -> Option<&ddb_parser::core::Core> {
    for entry in ddb {
        if let Device::Core { arguments } = entry.1 {
            return Some(arguments);
        }
    }

    None
}

/// First matching TtlClockGen (if any).
///
/// # Arguments
/// - `key` - target device key.
/// - `ddb` - device DB to search in.
pub(crate) fn ttl_clock_gen<'a, 'b>(
    key: &'a str,
    ddb: &'b DeviceDb,
) -> Option<&'b ddb_parser::ttl::TtlClockGen> {
    for entry in ddb {
        match entry {
            (ddb_key, Device::TtlClockGen { arguments }) if key == ddb_key => {
                return Some(arguments)
            }
            _ => continue,
        }
    }

    None
}

/// First matching TtlOut (if any).
pub(crate) fn ttl_out<'a>(key: &str, ddb: &'a DeviceDb) -> Option<&'a ddb_parser::ttl::TtlOut> {
    for entry in ddb {
        match entry {
            (ddb_key, Device::TtlOut { arguments }) if key == ddb_key => return Some(arguments),
            _ => continue,
        }
    }

    None
}

/// All channels for a given Urukul board, identified by CPLD device name.
///
/// This only searches for AD9910 channels.
pub(crate) fn urukul_channels<'a>(
    cpld_key: &str,
    ddb: &'a DeviceDb,
) -> impl Iterator<Item = &'a ddb_parser::urukul::Ad9910> {
    ddb.iter()
        .filter_map(|entry| match entry.1 {
            Device::Ad9910 { arguments } if arguments.cpld_device == cpld_key => Some(arguments),
            _ => None,
        })
        .sorted_by_key(|entry| entry.chip_select)
}
