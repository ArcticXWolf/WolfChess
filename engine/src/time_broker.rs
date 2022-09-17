use vampirc_uci::UciTimeControl;

struct TimeBroker {}

impl TimeBroker {
    pub fn new() -> TimeBroker {
        TimeBroker {}
    }

    pub fn seed_time_control(&self, time_control: UciTimeControl) {}

    pub fn start_timer(&self) {}

    pub fn send_stop(&self) {}
}
