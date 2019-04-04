use super::envelope::Envelope;
use super::frame_counter::FrameCounter;
use super::frame_counter::SequencerMode;
use super::length_counter::LengthCounter;
use super::timer::Timer;

static PERIOD_TABLE: [u16; 0x10] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

struct Sequencer {
    state: u16,
    mode_flag: bool,
}

pub struct Noise {
    timer: Timer,
    sequencer: Sequencer,
    frame_counter: FrameCounter,
    length_counter: LengthCounter,
    envelope: Envelope,
}

impl Sequencer {
    fn new() -> Sequencer {
        Sequencer {
            state: 1,
            mode_flag: false,
        }
    }

    fn tick(&mut self) {
        let xor_bit = if self.mode_flag { 6 } else { 1 };
        let x = self.state & 0x1;
        let y = (self.state >> xor_bit) & 0x1;
        let feedback = x ^ y;
        self.state = (feedback << 14) | (self.state >> 1);
    }

    fn sample(&self) -> f32 {
        match self.state & 0x1 {
            1 => 1.0,
            0 => -1.0,
            _ => unreachable!(),
        }
    }

    fn set_mode_flag(&mut self, flag: bool) {
        self.mode_flag = flag;
    }
}

impl Noise {
    pub fn new() -> Noise {
        Noise {
            timer: Timer::new(0),
            sequencer: Sequencer::new(),
            frame_counter: FrameCounter::new(),
            length_counter: LengthCounter::new(),
            envelope: Envelope::new(),
        }
    }

    pub fn tick(&mut self) {
        if self.timer.tick() {
            self.sequencer.tick();
        }

        // this needs to be before tick to handle signal by store.
        self.handle_frame_signal();
        self.frame_counter.tick();
    }

    pub fn handle_frame_signal(&mut self) {
        if self.frame_counter.is_quarter_frame() {
            self.envelope.tick();
        }
        if self.frame_counter.is_half_frame() {
            self.length_counter.tick();
        }
    }

    fn is_mute(&self) -> bool {
        self.length_counter.counter() == 0
    }

    fn volume(&self) -> f32 {
        if self.is_mute() {
            0.0
        } else {
            self.envelope.volume()
        }
    }

    pub fn sample(&self) -> f32 {
        self.volume() * self.sequencer.sample()
    }

    pub fn store(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => {
                let halt = ((val >> 5) & 0x1) != 0;
                self.length_counter.set_halt(halt);
                self.envelope.set_loop_flag(halt);
                let constant = ((val >> 4) & 0x1) != 0;
                self.envelope.set_constant_flag(constant);
                let volume = val & 0xf;
                self.envelope.set_volume(volume);
            }
            0x01 => {}
            0x02 => {
                let mode_flag = (val & 0x80) != 0;
                let period = PERIOD_TABLE[(val & 0xf) as usize];
                self.sequencer.set_mode_flag(mode_flag);
                self.timer.set_period(period);
            }
            0x03 => {
                let length_index = val >> 3;
                self.length_counter.load_with_index(length_index);
                self.envelope.set_start_flag();
            }
            0x15 => {
                let enabled = (val & 0x01) != 0;
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
