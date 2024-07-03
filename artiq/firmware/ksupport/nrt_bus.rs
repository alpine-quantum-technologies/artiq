pub mod i2c {
    use kernel_proto::*;
    use recv;
    use send;

    /// I2C multiplexer.
    #[derive(Debug)]
    pub struct Switch {
        pub busno: i32,
        pub address: i32,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Error {
        DeviceSelectionFailed,
        DataSelectionFailed,
        ReadSelectionFailed,
        WriteFailed,
        PollFailed,
    }

    impl Switch {
        /// Select (enable) a channel.
        pub fn select(&self, channel: u8) {
            switch_select(self.busno, self.address >> 1, 1 << channel)
        }

        /// Unselect all channels.
        pub fn reset(&self) {
            switch_select(self.busno, self.address >> 1, 0)
        }
    }

    pub extern "C" fn start(busno: i32) {
        send(&I2cStartRequest {
            busno: busno as u32,
        });
        recv!(&I2cBasicReply { succeeded } => if !succeeded {
            raise!("I2CError", "I2C bus could not be accessed");
        });
    }

    pub extern "C" fn restart(busno: i32) {
        send(&I2cRestartRequest {
            busno: busno as u32,
        });
        recv!(&I2cBasicReply { succeeded } => if !succeeded {
            raise!("I2CError", "I2C bus could not be accessed");
        });
    }

    pub extern "C" fn stop(busno: i32) {
        send(&I2cStopRequest {
            busno: busno as u32,
        });
        recv!(&I2cBasicReply { succeeded } => if !succeeded {
            raise!("I2CError", "I2C bus could not be accessed");
        });
    }

    pub extern "C" fn write(busno: i32, data: i32) -> bool {
        send(&I2cWriteRequest {
            busno: busno as u32,
            data: data as u8,
        });
        recv!(&I2cWriteReply { succeeded, ack } => {
            if !succeeded {
                raise!("I2CError", "I2C bus could not be accessed");
            }
            ack
        })
    }

    pub extern "C" fn read(busno: i32, ack: bool) -> i32 {
        send(&I2cReadRequest {
            busno: busno as u32,
            ack: ack,
        });
        recv!(&I2cReadReply { succeeded, data } => {
            if !succeeded {
                raise!("I2CError", "I2C bus could not be accessed");
            }
            data
        }) as i32
    }

    pub extern "C" fn switch_select(busno: i32, address: i32, mask: i32) {
        send(&I2cSwitchSelectRequest {
            busno: busno as u32,
            address: address as u8,
            mask: mask as u8,
        });
        recv!(&I2cBasicReply { succeeded } => { if !succeeded {
                raise!("I2CError", "I2C bus could not be accessed");
            }
        });
    }

    /// Poll a I2C device.
    ///
    /// # Arguments
    ///
    /// * `busno` - i2c bus number.
    /// * `busaddr` - address of the device on the i2c bus.
    pub fn poll(busno: i32, busaddr: i32) -> Result<(), Error> {
        start(busno);
        let ack = write(busno, busaddr);
        stop(busno);

        if !ack {
            Err(Error::PollFailed)
        } else {
            Ok(())
        }
    }

    /// Write bytes to a I2C device.
    ///
    /// # Arguments
    ///
    /// * `busno` - i2c bus number.
    /// * `busaddr` - address of the device on the i2c bus.
    /// * `addr` - address of the first byte to write on the device.
    /// * `bytes` - bytes to write.
    /// * `ack_last` - expect i2c ACK on the last byte written.
    pub fn write_bytes(
        busno: i32,
        busaddr: i32,
        addr: i32,
        data: &[u8],
        ack_last: bool,
    ) -> Result<(), Error> {
        let n = data.len();

        start(busno);
        if !write(busno, busaddr) {
            stop(busno);
            return Err(Error::DeviceSelectionFailed);
        }

        if !write(busno, addr) {
            stop(busno);
            return Err(Error::DataSelectionFailed);
        }

        for (i, &data) in data.iter().enumerate() {
            if !write(busno, data.into()) && (i < n - 1 || ack_last) {
                stop(busno);
                return Err(Error::WriteFailed);
            }
        }

        stop(busno);
        Ok(())
    }

    /// Read N bytes from a I2C device.
    ///
    /// # Arguments
    ///
    /// * `busno` - i2c bus number.
    /// * `busaddr` - address of the target device on the i2c bus.
    /// * `addr` - address of the first byte to read on the device.
    pub fn read_bytes<const N: usize>(
        busno: i32,
        busaddr: i32,
        addr: i32,
    ) -> Result<[u8; N], Error> {
        start(busno);
        if !write(busno, busaddr) {
            stop(busno);
            return Err(Error::DeviceSelectionFailed);
        }

        if !write(busno, addr) {
            stop(busno);
            return Err(Error::DataSelectionFailed);
        }

        restart(busno);
        if !write(busno, busaddr | 1) {
            stop(busno);
            return Err(Error::ReadSelectionFailed);
        }

        let mut bytes = [0; N];
        for (i, data) in bytes.iter_mut().enumerate() {
            *data = read(busno, i < N - 1) as u8;
        }

        stop(busno);
        Ok(bytes)
    }
}

pub mod kasli_i2c {
    use super::i2c;
    use sinara_config::i2c::KasliPort;

    /// Kasli I2C bus.
    #[derive(Debug)]
    pub struct Bus {
        pub switch0: i2c::Switch,
        pub switch1: i2c::Switch,
    }

    impl Bus {
        const EEPROM_ADDR: i32 = 0xa0;

        pub fn eeprom_read_i32(&self, port: KasliPort, addr: i32) -> Result<i32, i2c::Error> {
            let _select = SelectDevice::new(port, &self.switch0, &self.switch1);

            let bytes = i2c::read_bytes::<4>(0, Self::EEPROM_ADDR, addr)?;
            Ok(i32::from_be_bytes(bytes))
        }

        pub fn eeprom_write_i32(
            &self,
            port: KasliPort,
            addr: i32,
            value: i32,
        ) -> Result<(), i2c::Error> {
            let _select = SelectDevice::new(port, &self.switch0, &self.switch1);

            let data = value.to_be_bytes();
            i2c::write_bytes(0, Self::EEPROM_ADDR, addr, &data, true)?;
            i2c::poll(0, Self::EEPROM_ADDR)
        }
    }

    struct SelectDevice<'a> {
        switch: &'a i2c::Switch,
        port: u8,
    }

    impl<'a> SelectDevice<'a> {
        fn new(port: KasliPort, switch0: &'a i2c::Switch, switch1: &'a i2c::Switch) -> Self {
            let port: u8 = port.into();

            let sel = if port < 8 {
                Self {
                    switch: switch0,
                    port,
                }
            } else {
                Self {
                    switch: switch1,
                    port: port - 8,
                }
            };

            sel.switch.select(sel.port);
            sel
        }
    }

    impl Drop for SelectDevice<'_> {
        fn drop(&mut self) {
            self.switch.reset();
        }
    }
}

pub mod spi {
    use kernel_proto::*;
    use recv;
    use send;

    pub extern "C" fn set_config(busno: i32, flags: i32, length: i32, div: i32, cs: i32) {
        send(&SpiSetConfigRequest {
            busno: busno as u32,
            flags: flags as u8,
            length: length as u8,
            div: div as u8,
            cs: cs as u8,
        });
        recv!(&SpiBasicReply { succeeded } => if !succeeded {
            raise!("SPIError", "SPI bus could not be accessed");
        });
    }

    pub extern "C" fn write(busno: i32, data: i32) {
        send(&SpiWriteRequest {
            busno: busno as u32,
            data: data as u32,
        });
        recv!(&SpiBasicReply { succeeded } => if !succeeded {
            raise!("SPIError", "SPI bus could not be accessed");
        });
    }

    pub extern "C" fn read(busno: i32) -> i32 {
        send(&SpiReadRequest {
            busno: busno as u32,
        });
        recv!(&SpiReadReply { succeeded, data } => {
            if !succeeded {
                raise!("SPIError", "SPI bus could not be accessed");
            }
            data
        }) as i32
    }
}
