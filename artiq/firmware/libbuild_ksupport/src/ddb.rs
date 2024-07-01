use ddb_parser::{Device, DeviceDb};

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
