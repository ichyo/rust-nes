use super::timer::Timer;

/// Waveform generator
struct Sequencer {
    clock: u8,
    duty: u8,
}

/// Triangle channel
pub struct Triangle {
    timer: Timer,
    sequencer: Sequencer,
}

static WAVEFORM: [u8; 32] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15,
];

const WAVE_LEN: u8 = 32;

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer { duty: 0, clock: 0 }
    }

    pub fn tick(&mut self) {
        self.clock = (self.clock + 1) % WAVE_LEN;
    }

    pub fn sample(&self) -> f32 {
        (2.0 * f32::from(WAVEFORM[self.clock as usize]) / 15.0) - 1.0
    }
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            timer: Timer::new(0),
            sequencer: Sequencer::new(),
        }
    }

    pub fn tick(&mut self) {
        if self.timer.tick() {
            self.sequencer.tick();
        }
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
            0x00 => {}
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
            }
            _ => unreachable!(),
        }
    }
}
