use self::tabsstate::TabsState;

pub mod tabsstate;

pub struct App<'a> {
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tabs: TabsState::new(vec![" Timer ", " Stats "]),
        }
    }
}
