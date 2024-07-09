pub mod i2c {
    use kernel_proto::*;
    use recv;
    use send;
    use sinara_config::i2c::KasliPort;

    /// I²C switch (multiplexer).
    #[derive(Debug)]
    pub struct Switch {
        pub busno: i32,
        pub address: i32,
    }

    /// Kasli I²C bus.
    #[derive(Debug)]
    pub struct KasliI2C {
        pub switch0: Switch,
        pub switch1: Switch,
    }

    /// High-level I²C driver errors.
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Error {
        DeviceSelectionFailed,
        DataSelectionFailed,
        ReadSelectionFailed,
        WriteFailed,
        PollFailed,
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

    impl Switch {
        /// Select a channel.
        pub fn select(&self, channel: u8) {
            switch_select(self.busno, self.address >> 1, 1 << channel)
        }

        /// De-select all channels.
        pub fn reset(&self) {
            switch_select(self.busno, self.address >> 1, 0)
        }
    }

    impl KasliI2C {
        const EEPROM_ADDR: i32 = 0xa0;

        /// Read four bytes from an EEPROM device.
        pub fn eeprom_read_i32(&self, port: KasliPort, addr: i32) -> Result<i32, Error> {
            let _select = SelectDevice::new(port, &self.switch0, &self.switch1);

            let bytes = read_bytes::<4>(0, Self::EEPROM_ADDR, addr)?;
            Ok(i32::from_be_bytes(bytes))
        }

        /// Write four bytes to an EEPROM device.
        pub fn eeprom_write_i32(
            &self,
            port: KasliPort,
            addr: i32,
            value: i32,
        ) -> Result<(), Error> {
            let _select = SelectDevice::new(port, &self.switch0, &self.switch1);

            let data = value.to_be_bytes();
            write_bytes(0, Self::EEPROM_ADDR, addr, &data, true)?;
            let _ = poll(0, Self::EEPROM_ADDR); // discard error

            Ok(())
        }
    }

    struct SelectDevice<'a> {
        switch: &'a Switch,
        port: u8,
    }

    impl<'a> SelectDevice<'a> {
        fn new(port: KasliPort, switch0: &'a Switch, switch1: &'a Switch) -> Self {
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

    impl<'a> Drop for SelectDevice<'a> {
        fn drop(&mut self) {
            self.switch.reset()
        }
    }

    /// Poll a I²C device.
    ///
    /// # Arguments
    ///
    /// * `busno` - I²C bus number.
    /// * `busaddr` - target device address.
    fn poll(busno: i32, busaddr: i32) -> Result<(), Error> {
        start(busno);
        let ack = write(busno, busaddr);
        stop(busno);

        if ack {
            Ok(())
        } else {
            Err(Error::PollFailed)
        }
    }

    /// Write bytes to a I²C device.
    ///
    /// # Arguments
    ///
    /// * `busno` - I²C bus number.
    /// * `busaddr` - target device address.
    /// * `addr` - data destination address on the target device.
    /// * `bytes` - data to write.
    /// * `ack_last` - expect I²C `ACK` on the last byte written.
    fn write_bytes(
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

    /// Read N bytes from a I²C device.
    ///
    /// # Arguments
    ///
    /// * `busno` - I²C bus number.
    /// * `busaddr` - target device address.
    /// * `addr` - data address on the target device.
    fn read_bytes<const N: usize>(busno: i32, busaddr: i32, addr: i32) -> Result<[u8; N], Error> {
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
