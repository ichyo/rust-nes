use super::timer::Timer;

/// Waveform generator
struct Sequencer {
    clock: u8,
    duty: u8,
}

/// Triangle channel
pub struct Pulse {
    timer: Timer,
    sequencer: Sequencer,
    clocks: u64,
}

static WAVEFORM: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

const WAVE_LEN: u8 = 8;

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer { duty: 0, clock: 0 }
    }

    pub fn tick(&mut self) {
        self.clock = (self.clock + 1) % WAVE_LEN;
    }

    pub fn set_duty(&mut self, duty: u8) {
        assert!(duty < 4);
        self.duty = duty;
    }

    pub fn reset(&mut self) {
        self.clock = 0;
    }

    fn waveform(&self) -> [u8; 8] {
        WAVEFORM[self.duty as usize]
    }

    pub fn sample(&self) -> f32 {
        match self.waveform()[self.clock as usize] {
            1 => 1.0,
            0 => -1.0,
            _ => unreachable!(),
        }
    }
}

impl Pulse {
    pub fn new() -> Pulse {
        Pulse {
            timer: Timer::new(0),
            sequencer: Sequencer::new(),
            clocks: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.clocks % 2 == 0 && self.timer.tick() {
            self.sequencer.tick();
        }
        self.clocks += 1;
    }

    fn is_mute(&self) -> bool {
        self.timer.period() < 8
    }

    pub fn sample(&self) -> f32 {
        if self.is_mute() {
            0.0
        } else {
            self.sequencer.sample()
        }
    }

    pub fn store(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => {
                let duty = (val >> 6) & 0x3;
                self.sequencer.set_duty(duty);
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
                self.sequencer.reset();
            }
            _ => unreachable!(),
        }
    }
}
