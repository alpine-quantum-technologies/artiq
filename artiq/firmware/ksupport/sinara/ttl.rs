use crate::rtio;

#[cfg_attr(not(any(has_sinara_ttl_out, has_sinara_led)), allow(dead_code))]
#[derive(Debug)]
pub struct TtlOut {
    pub channel: i32,
}

#[cfg_attr(not(has_sinara_ttl_clk_gen), allow(dead_code))]
#[derive(Debug)]
pub struct TtlClockGen {
    pub channel: i32,
    pub acc_width: i64,
}

#[cfg_attr(not(any(has_sinara_ttl_out, has_sinara_led)), allow(dead_code))]
impl TtlOut {
    #[allow(dead_code)]
    pub fn output(&self) {}

    #[inline]
    pub fn on(&self) {
        self.set_o(true)
    }

    #[inline]
    pub fn off(&self) {
        self.set_o(false)
    }

    #[inline]
    pub fn pulse_mu(&self, duration: i64) {
        self.on();
        rtio::delay_mu(duration);
        self.off();
    }

    #[inline(always)]
    fn set_o(&self, o: bool) {
        unsafe { crate::RTIO_OUTPUT_FN(self.channel << 8, if o { 1 } else { 0 }) }
    }
}

#[cfg_attr(not(has_sinara_ttl_clk_gen), allow(dead_code))]
impl TtlClockGen {
    #[inline(always)]
    pub fn set_mu(&self, ftw: i32) {
        unsafe { crate::RTIO_OUTPUT_FN(self.channel << 8, ftw) }
    }
}
