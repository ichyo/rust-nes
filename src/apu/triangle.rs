use super::frame_counter::FrameCounter;
use super::frame_counter::SequencerMode;
use super::length_counter::LengthCounter;
use super::timer::Timer;

/// Waveform generator
struct Sequencer {
    clock: u8,
}

struct LinearCounter {
    reload_value: u8,
    reload_flag: bool,
    control_flag: bool,
    counter: u8,
}

/// Triangle channel
pub struct Triangle {
    timer: Timer,
    sequencer: Sequencer,
    frame_counter: FrameCounter,
    length_counter: LengthCounter,
    linear_counter: LinearCounter,
}

static WAVEFORM: [u8; 32] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15,
];

const WAVE_LEN: u8 = 32;

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer { clock: 0 }
    }

    pub fn tick(&mut self) {
        self.clock = (self.clock + 1) % WAVE_LEN;
    }

    pub fn sample(&self) -> f32 {
        (2.0 * f32::from(WAVEFORM[self.clock as usize]) / 15.0) - 1.0
    }
}

impl LinearCounter {
    fn new() -> LinearCounter {
        LinearCounter {
            reload_value: 0,
            reload_flag: false,
            control_flag: false,
            counter: 0,
        }
    }
    fn tick(&mut self) {
        if self.reload_flag {
            self.counter = self.reload_value;
        } else if self.counter > 0 {
            self.counter -= 1;
        }
        if !self.control_flag {
            self.reload_flag = false;
        }
    }

    fn set_control_flag(&mut self, flag: bool) {
        self.control_flag = flag;
    }

    fn set_reload_flag(&mut self) {
        self.reload_flag = true;
    }

    fn set_reload_value(&mut self, value: u8) {
        self.reload_value = value;
    }

    fn counter(&self) -> u8 {
        self.counter
    }
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            timer: Timer::new(0),
            sequencer: Sequencer::new(),
            frame_counter: FrameCounter::new(),
            length_counter: LengthCounter::new(),
            linear_counter: LinearCounter::new(),
        }
    }

    pub fn tick(&mut self) {
        // CPU clocks
        if self.timer.tick() {
            // timer clock
            // The sequencer is clocked by the timer as long as both the linear counter and the length counter are nonzero.
            if self.linear_counter.counter() != 0 && self.length_counter.counter() != 0 {
                self.sequencer.tick();
            }
        }

        // this needs to be before tick to handle signal by store.
        self.handle_frame_signal();
        self.frame_counter.tick();
    }

    pub fn handle_frame_signal(&mut self) {
        if self.frame_counter.is_quarter_frame() {
            self.linear_counter.tick();
        }
        if self.frame_counter.is_half_frame() {
            self.length_counter.tick();
        }
    }

    pub fn sample(&self) -> f32 {
        self.sequencer.sample()
    }

    pub fn store(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => {
                let halt = ((val >> 7) & 0x1) != 0;
                self.length_counter.set_halt(halt);
                self.linear_counter.set_control_flag(halt);
                let reload_value = val & 0x7f;
                self.linear_counter.set_reload_value(reload_value);
            }
            0x01 => {}
            0x02 => {
                let old_period = self.timer.period();
                let new_period = (old_period & 0x700) | u16::from(val);
                self.timer.set_period(new_period);
            }
            0x03 => {
                let old_period = self.timer.period();
                let new_period = (old_period & 0xff) | (u16::from(val & 0x7) << 8);
                self.timer.set_period(new_period);
                let length_index = val >> 3;
                self.length_counter.load_with_index(length_index);
                self.linear_counter.set_reload_flag();
            }
            0x15 => {
                let enabled = (val & 0x1) != 0;
                self.length_counter.set_enabled(enabled);
            }
            0x17 => {
                let mode = if (val & 0x80) != 0 {
                    SequencerMode::FiveStep
                } else {
                    SequencerMode::FourStep
                };
                self.frame_counter.set_mode(mode);
            }
            _ => unreachable!(),
        }
    }
}
