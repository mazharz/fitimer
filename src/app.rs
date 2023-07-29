use self::{stat::Stat, timer::Timer};

pub mod stat;
pub mod timer;

pub struct App {
    pub timer: Timer,
    pub stat: Stat,
}

impl App {
    pub fn new() -> App {
        App {
            timer: Timer::new(false),
            stat: Stat::init(),
        }
    }

    pub fn on_tick(&mut self) {}
}
