#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SequencerMode {
    FourStep,
    FiveStep,
}

impl SequencerMode {
    fn period(self) -> u16 {
        match self {
            SequencerMode::FourStep => 29830,
            SequencerMode::FiveStep => 37282,
        }
    }

    fn is_quarter_frame(self, cycle: u16) -> bool {
        match (self, cycle) {
            (SequencerMode::FourStep, 7457) => true,
            (SequencerMode::FourStep, 14913) => true,
            (SequencerMode::FourStep, 22371) => true,
            (SequencerMode::FourStep, 29829) => true,
            (SequencerMode::FiveStep, 7457) => true,
            (SequencerMode::FiveStep, 14913) => true,
            (SequencerMode::FiveStep, 22371) => true,
            (SequencerMode::FiveStep, 37281) => true,
            _ => false,
        }
    }

    fn is_half_frame(self, cycle: u16) -> bool {
        match (self, cycle) {
            (SequencerMode::FourStep, 14913) => true,
            (SequencerMode::FourStep, 29829) => true,
            (SequencerMode::FiveStep, 14913) => true,
            (SequencerMode::FiveStep, 37281) => true,
            _ => false,
        }
    }
}

pub struct FrameCounter {
    mode: SequencerMode,
    clocks: u16,
    quarter_frame: bool,
    half_frame: bool,
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        FrameCounter {
            mode: SequencerMode::FourStep,
            clocks: 0,
            quarter_frame: false,
            half_frame: false,
        }
    }

    /// CPU Clock
    pub fn tick(&mut self) {
        self.clocks += 1;
        if self.clocks >= self.mode.period() {
            self.clocks = 0;
        }
        self.quarter_frame = self.mode.is_quarter_frame(self.clocks);
        self.half_frame = self.mode.is_half_frame(self.clocks);
    }

    pub fn is_quarter_frame(&self) -> bool {
        self.quarter_frame
    }

    pub fn is_half_frame(&self) -> bool {
        self.half_frame
    }

    pub fn set_mode(&mut self, mode: SequencerMode) {
        self.mode = mode;
        // After 3 or 4 CPU clock cycles, the timer is reset.
        // TODO: it's reset immediately now.
        self.clocks = 0;
        // If the mode flag is set,
        // then both "quarter frame" and "half frame"
        // signals are also generated.
        self.quarter_frame = mode == SequencerMode::FiveStep;
        self.half_frame = mode == SequencerMode::FiveStep;
    }
}
