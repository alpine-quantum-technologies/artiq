use crate::rtio;

pub struct TtlOut {
    pub channel: i32,
}

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
