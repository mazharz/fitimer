use self::{help::Help, stat::Stat, timer::Timer};

pub mod help;
pub mod stat;
pub mod timer;

pub struct App {
    pub timer: Timer,
    pub stat: Stat,
    pub help: Help,
}

impl App {
    pub fn new() -> App {
        App {
            timer: Timer::new(false),
            stat: Stat::init(),
            help: Help::new(),
        }
    }

    pub fn on_tick(&mut self) {}
}
