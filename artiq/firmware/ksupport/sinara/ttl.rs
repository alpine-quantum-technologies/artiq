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

    pub fn on(&self) {
        self.set_o(true)
    }

    pub fn off(&self) {
        self.set_o(false)
    }

    pub fn pulse_mu(&self, duration: i64) {
        self.on();
        rtio::delay_mu(duration);
        self.off();
    }

    fn set_o(&self, o: bool) {
        rtio::output(self.channel << 8, if o { 1 } else { 0 })
    }
}

#[cfg_attr(not(has_sinara_ttl_clk_gen), allow(dead_code))]
impl TtlClockGen {
    pub fn set_mu(&self, ftw: i32) {
        rtio::output(self.channel << 8, ftw)
    }

    pub fn stop(&self) {
        self.set_mu(0)
    }
}
