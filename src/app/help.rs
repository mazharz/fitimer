pub struct Help {
    pub is_open: bool,
}

impl Help {
    pub fn new() -> Help {
        Help { is_open: false }
    }

    pub fn toggle_is_open(&mut self) {
        self.is_open = !self.is_open;
    }
}
