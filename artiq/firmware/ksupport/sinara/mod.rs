#[cfg(not(has_rtio))]
compile_error!("Need RTIO to use Sinara drivers");

pub mod ttl;

include!(concat!(env!("OUT_DIR"), "/peripherals.rs"));

/* Syscalls */
#[cfg(has_sinara_ttl_out)]
pub extern "C" fn ttl_out_on(channel: usize) {
    PERIPHERALS.ttl_out[channel].on()
}

#[cfg(has_sinara_ttl_out)]
pub extern "C" fn ttl_out_off(channel: usize) {
    PERIPHERALS.ttl_out[channel].off()
}

pub extern "C" fn ttl_out_count() -> usize {
    PERIPHERALS.ttl_out.len()
}
