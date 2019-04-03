mod pulse;
mod timer;

use self::pulse::Pulse;
use std::collections::vec_deque::Drain;
use std::collections::VecDeque;

use log::trace;

const CPU_CLOCKS: u64 = 1_789_733;
const SAMPLE_RATE: u64 = 44_100;
const COUNTS_PER_SEC: u64 = CPU_CLOCKS * SAMPLE_RATE;
const BUFFER_LENGTH: usize = 1024;

/// audio processing unit.
pub struct Apu {
    pulse1: Pulse,
    pulse2: Pulse,
    counts: u64,
    buffer: VecDeque<f32>,
}

impl Default for Apu {
    fn default() -> Apu {
        Apu::new()
    }
}

impl Apu {
    /// Create APU
    pub fn new() -> Apu {
        Apu {
            pulse1: Pulse::new(),
            pulse2: Pulse::new(),
            counts: 0,
            buffer: VecDeque::with_capacity(BUFFER_LENGTH),
        }
    }

    /// load interface exposed to cpu via bus
    pub fn load(&self, addr: u16) -> u8 {
        trace!("Load addr={:#x}", addr);
        0
    }

    /// store interface exposed to cpu via bus
    pub fn store(&mut self, addr: u16, val: u8) {
        trace!("Store addr={:#x} val={:#x}", addr, val);
        match addr {
            0x00...0x03 => self.pulse1.store(addr, val),
            0x04...0x07 => self.pulse2.store(addr - 0x04, val),
            _ => {}
        }
    }

    fn sample(&self) -> f32 {
        let p1 = self.pulse1.sample();
        let p2 = self.pulse2.sample();
        p1 * 0.3 + p2 * 0.3
    }

    /// Tick 1 CPU clock
    pub fn tick(&mut self) {
        self.pulse1.tick();
        self.pulse2.tick();
        let k1 = self.counts / (COUNTS_PER_SEC / SAMPLE_RATE);
        self.counts += COUNTS_PER_SEC / CPU_CLOCKS;
        let k2 = self.counts / (COUNTS_PER_SEC / SAMPLE_RATE);
        if k1 != k2 {
            self.append_buffer(self.sample());
        }
        if self.counts == COUNTS_PER_SEC {
            self.counts = 0;
        }
    }

    fn append_buffer(&mut self, x: f32) {
        if self.buffer.len() == BUFFER_LENGTH {
            self.buffer.pop_front();
        }
        self.buffer.push_back(x);
    }

    /// Get sampling buffer
    pub fn consume_buffer(&mut self) -> Drain<f32> {
        self.buffer.drain(..)
    }
}
