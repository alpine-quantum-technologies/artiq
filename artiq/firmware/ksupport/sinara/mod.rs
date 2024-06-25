#[cfg(not(has_rtio))]
compile_error!("Need RTIO to use Sinara drivers");

pub mod ttl;

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
