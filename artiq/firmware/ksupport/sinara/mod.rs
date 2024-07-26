use core::convert::TryInto;

#[cfg(not(has_rtio))]
compile_error!("Need RTIO to use Sinara drivers");

mod ttl;
mod urukul;

#[cfg(has_sinara_urukul)]
use crate::nrt_bus::i2c;
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
    PERIPHERALS.urukul[board].init(false).is_ok()
}

pub extern "C" fn urukul_channel_init(board: usize, channel: u8) -> bool {
    PERIPHERALS.urukul[board]
        .channel(channel.try_into().unwrap())
        .init(false)
        .is_ok()
}

pub extern "C" fn urukul_channel_setup_sync(
    board: usize,
    channel: u8,
    sync_data: *mut urukul::SyncData,
) -> bool {
    if sync_data.is_null() {
        return false;
    }

    let board = &PERIPHERALS.urukul[board];
    let channel = board.channel(channel.try_into().unwrap());

    if let Ok(data) = channel.read_sync_data() {
        unsafe { *sync_data = data };
        channel.setup_sync(&data).is_ok()
    } else {
        false
    }
}

pub extern "C" fn urukul_write_coarse_attenuation(board: usize, att_mu: u32) -> bool {
    PERIPHERALS.urukul[board]
        .write_attenuation_register(att_mu)
        .is_ok()
}

pub extern "C" fn urukul_read_coarse_attenuation(board: usize, att_mu: *mut u32) -> bool {
    if let Ok(att) = PERIPHERALS.urukul[board].read_attenuation_register() {
        unsafe { *att_mu = att };
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

pub extern "C" fn urukul_channel_set_mu_coherent(
    board: usize,
    channel: u8,
    ftw: u32,
    pow: *mut u16,
    asf: u16,
    ref_time_mu: i64,
    io_update_delay_mu: i64,
) -> bool {
    if pow.is_null() {
        // TODO: log error
        return false;
    }

    if let Ok(new_pow) = PERIPHERALS.urukul[board]
        .channel(channel.try_into().unwrap())
        .set_mu_coherent(ftw, unsafe { *pow }, asf, ref_time_mu, io_update_delay_mu)
    {
        unsafe { *pow = new_pow };
        true
    } else {
        false
    }
}
