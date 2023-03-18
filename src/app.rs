use self::{tabsstate::TabsState, timerstate::TimerState};

pub mod tabsstate;
pub mod timerstate;

pub struct App<'a> {
    pub tabs: TabsState<'a>,
    pub timer: TimerState,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tabs: TabsState::new(vec![" Timer ", " Stats "]),
            timer: TimerState::new(false),
        }
    }

    pub fn on_tick(&mut self) {}
}
