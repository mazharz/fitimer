use crate::{config::Config, expiry::Expiry};

pub struct TimerState {
    pub enabled: bool,
    pub expiry: Expiry,
}

impl TimerState {
    pub fn new(enabled: bool) -> TimerState {
        let expiry = get_init_expiry();

        TimerState { enabled, expiry }
    }

    fn reset(&mut self) {
        self.expiry = get_init_expiry();
    }

    pub fn toggle_enabled(&mut self) {
        self.reset();
        self.enabled = !self.enabled;
    }
}

fn get_init_expiry() -> Expiry {
    let init_duration = Config::read().app.durations.work;
    let init_duration = Expiry::new(init_duration);

    return init_duration;
}
