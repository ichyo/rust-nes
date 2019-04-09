#[derive(Clone)]
/// Variable-rate timer
pub struct Timer {
    period: u16,
    clocks: u16,
}

impl Timer {
    /// Create new timer
    pub fn new(period: u16) -> Timer {
        Timer { period, clocks: 0 }
    }

    pub fn period(&self) -> u16 {
        self.period
    }

    pub fn set_period(&mut self, period: u16) {
        self.period = period;
        self.clocks = 0;
    }

    /// tick a clock. true if it output clocks
    pub fn tick(&mut self) -> bool {
        if self.clocks >= self.period {
            self.clocks = 0;
            true
        } else {
            self.clocks += 1;
            false
        }
    }
}
