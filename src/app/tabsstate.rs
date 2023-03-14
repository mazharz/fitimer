pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub active_index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState {
            titles,
            active_index: 0,
        }
    }

    pub fn next(&mut self) {
        self.active_index = (self.active_index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.active_index > 0 {
            self.active_index -= 1;
        } else {
            self.active_index = self.titles.len() - 1;
        }
    }
}
