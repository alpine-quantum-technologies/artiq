use crate::rtio;
use sinara_config::edge_counter::Config;

#[cfg_attr(not(has_sinara_edge_counter), allow(dead_code))]
#[derive(Debug)]
pub struct EdgeCounter {
    pub channel: i32,
    pub gateware_width: i32,
}

#[cfg_attr(not(has_sinara_edge_counter), allow(dead_code))]
impl EdgeCounter {
    /// Close the counter gate.
    #[inline]
    pub fn stop_gate(&self) {
        self.set_config(Config::SendCountEvent);
    }

    /// Open the counter gate, count rising edges.
    #[inline]
    pub fn start_gate_rising(&self) {
        self.set_config(Config::CountRising | Config::ResetToZero);
    }

    /// Wait for and return count total from previously requested input event.
    ///
    /// If is valid to trigger multiple gate periods without immediately
    /// reading back the count total. The results will be returned in
    /// order on subsequent fetch calls.
    ///
    /// This function blocks until a result becomes available.
    ///
    /// Returns -1 if the counter overflowed.
    #[inline]
    pub fn fetch_count(&self) -> i32 {
        let counter_max = (1 << (self.gateware_width - 1)) - 1;

        let count = rtio::input_data(self.channel);
        if count == counter_max {
            -1
        } else {
            count
        }
    }

    #[inline(always)]
    fn set_config(&self, config: Config) {
        unsafe { crate::RTIO_OUTPUT_FN(self.channel << 8, config.bits()) }
    }
}
