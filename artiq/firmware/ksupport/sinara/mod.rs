use core::convert::TryInto;

#[cfg(not(has_rtio))]
compile_error!("Need RTIO to use Sinara drivers");

mod ttl;
mod urukul;

#[cfg(has_sinara_urukul)]
use crate::rtio;
#[cfg(has_sinara_urukul)]
use crate::spi2;

include!(concat!(env!("OUT_DIR"), "/peripherals.rs"));

/* Syscalls */
pub extern "C" fn ttl_out_on(channel: usize) {
    PERIPHERALS.ttl_out[channel].on()
}

pub extern "C" fn ttl_out_off(channel: usize) {
    PERIPHERALS.ttl_out[channel].off()
}

pub extern "C" fn ttl_out_count() -> usize {
    PERIPHERALS.ttl_out.len()
}

pub extern "C" fn led_on(channel: usize) {
    PERIPHERALS.led[channel].on()
}

pub extern "C" fn led_off(channel: usize) {
    PERIPHERALS.led[channel].off()
}

pub extern "C" fn led_count() -> usize {
    PERIPHERALS.led.len()
}

pub extern "C" fn urukul_count() -> usize {
    PERIPHERALS.urukul.len()
}

pub extern "C" fn urukul_init(board: usize) -> bool {
    match PERIPHERALS.urukul[board].init(false) {
        Ok(_) => true,
        Err(e) => {
            core_log!(log::Level::Error, "Urukul board init failed: {:?}", e);
            rtio::break_realtime();
            false
        }
    }
}

pub extern "C" fn urukul_channel_init(board: usize, channel: u8) -> bool {
    match PERIPHERALS.urukul[board]
        .channel(channel.try_into().unwrap())
        .init(false)
    {
        Ok(_) => true,
        Err(e) => {
            core_log!(log::Level::Error, "Urukul channel init failed: {:?}", e);
            rtio::break_realtime();
            false
        }
    }
}

pub extern "C" fn urukul_write_coarse_attenuation(board: usize, att_mu: u32) -> bool {
    PERIPHERALS.urukul[board]
        .write_attenuation_register(att_mu)
        .is_ok()
}

pub extern "C" fn urukul_read_coarse_attenuation(board: usize, att_mu: *mut u32) -> bool {
    if let Ok(att) = PERIPHERALS.urukul[board].read_attenuation_register() {
        unsafe {
            *att_mu = att;
        }
        true
    } else {
        false
    }
}

pub extern "C" fn urukul_channel_rf_on(board: usize, channel: u8) {
    PERIPHERALS.urukul[board]
        .channel(channel.try_into().unwrap())
        .switch_on()
}

pub extern "C" fn urukul_channel_rf_off(board: usize, channel: u8) {
    PERIPHERALS.urukul[board]
        .channel(channel.try_into().unwrap())
        .switch_off()
}

pub extern "C" fn urukul_channel_set_mu(
    board: usize,
    channel: u8,
    ftw: u32,
    pow: u16,
    asf: u16,
) -> bool {
    PERIPHERALS.urukul[board]
        .channel(channel.try_into().unwrap())
        .set_mu(ftw, pow, asf)
        .is_ok()
}
