use crate::rtio;

#[cfg_attr(not(any(has_sinara_ttl_out, has_sinara_led)), allow(dead_code))]
pub struct TtlOut {
    pub channel: i32,
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

    fn set_o(&self, o: bool) {
        rtio::output(self.channel << 8, if o { 1 } else { 0 })
    }
}
