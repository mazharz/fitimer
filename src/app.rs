use self::timerstate::TimerState;

pub mod timerstate;

pub struct App {
    pub timer: TimerState,
}

impl App {
    pub fn new() -> App {
        App {
            timer: TimerState::new(false),
        }
    }

    pub fn on_tick(&mut self) {}
}
