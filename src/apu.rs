mod frame_counter;
mod length_counter;
mod pulse;
mod timer;
mod triangle;

use self::pulse::Pulse;
use self::triangle::Triangle;
use std::collections::vec_deque::Drain;
use std::collections::VecDeque;

use log::trace;

const CPU_CLOCK_RATE: u64 = 1_789_733;
const SAMPLE_RATE: u64 = 44_100;
const BUFFER_LENGTH: usize = 1024;

fn sample_index(clocks: u64) -> u64 {
    clocks * SAMPLE_RATE / CPU_CLOCK_RATE
}

/// audio processing unit.
pub struct Apu {
    pulse1: Pulse,
    pulse2: Pulse,
    triangle: Triangle,
    clocks: u64,
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
            triangle: Triangle::new(),
            clocks: 0,
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
            0x08...0x0b => self.triangle.store(addr - 0x08, val),
            0x15 => {
                self.pulse1.store(addr, val);
                self.pulse2.store(addr, val >> 1);
                self.triangle.store(addr, val >> 2);
            }
            0x17 => {
                self.pulse1.store(addr, val);
                self.pulse2.store(addr, val);
                self.triangle.store(addr, val);
            }
            _ => {}
        }
    }

    fn sample(&self) -> f32 {
        let p1 = self.pulse1.sample();
        let p2 = self.pulse2.sample();
        let t = self.triangle.sample();
        p1 * 0.2 + p2 * 0.2 + t * 0.4
    }

    /// Tick 1 CPU clock
    pub fn tick(&mut self) {
        self.pulse1.tick();
        self.pulse2.tick();
        self.triangle.tick();
        if sample_index(self.clocks) != sample_index(self.clocks + 1) {
            self.append_buffer(self.sample());
        }
        self.clocks += 1;
        if self.clocks == CPU_CLOCK_RATE {
            self.clocks = 0;
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
