use crate::{duration::Duration, env};

pub struct TimerState {
    pub enabled: bool,
    pub current_remaining: Duration,
}

impl TimerState {
    pub fn new(enabled: bool) -> TimerState {
        let init_duration = env::var(String::from("DURATION_WORK"));
        let init_duration = init_duration.unwrap_or(String::from("25"));
        let init_duration = init_duration
            .parse::<u64>()
            .expect("Couldn't convert duration string into number");
        let init_duration = Duration::new(init_duration);

        TimerState {
            enabled,
            current_remaining: init_duration,
        }
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }
}
