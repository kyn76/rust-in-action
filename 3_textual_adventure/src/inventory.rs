pub struct Inventory {
    pub has_key: bool,
    pub has_sword: bool,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            has_key: false,
            has_sword: false,
        }
    }

    pub fn add_key(&mut self) {
        self.has_key = true
    }

    pub fn add_sword(&mut self) {
        self.has_sword = true
    }
}