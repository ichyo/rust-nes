pub struct Divider {
    reload_value: u8,
    counter: u8,
}

impl Divider {
    pub fn new() -> Divider {
        Divider {
            reload_value: 0,
            counter: 0,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.counter == 0 {
            self.counter = self.reload_value;
            true
        } else {
            self.counter -= 1;
            false
        }
    }

    pub fn reset(&mut self) {
        self.counter = self.reload_value;
    }

    pub fn set_reload_value(&mut self, value: u8) {
        self.reload_value = value;
    }
}
