mod divider;
mod envelope;
mod frame_counter;
mod length_counter;
mod noise;
mod pulse;
mod sweep;
mod timer;
mod triangle;

use self::noise::Noise;
use self::pulse::Pulse;
use self::pulse::PulseId;
use self::triangle::Triangle;
use std::collections::vec_deque::Drain;
use std::collections::VecDeque;

use log::warn;

const CPU_CLOCK_RATE: u64 = 1_789_733;
const SAMPLE_RATE: u64 = 44_100;
const BUFFER_LENGTH: usize = 1024;

fn sample_index(clocks: u64) -> u64 {
    clocks * SAMPLE_RATE / CPU_CLOCK_RATE
}

#[derive(Clone)]
/// audio processing unit.
pub struct Apu {
    pulse1: Pulse,
    pulse2: Pulse,
    triangle: Triangle,
    noise: Noise,
    clocks: u64,
    buffer: VecDeque<(f32, f32)>,
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
            pulse1: Pulse::new(PulseId::One),
            pulse2: Pulse::new(PulseId::Two),
            triangle: Triangle::new(),
            noise: Noise::new(),
            clocks: 0,
            buffer: VecDeque::with_capacity(BUFFER_LENGTH),
        }
    }

    /// load interface exposed to cpu via bus
    pub fn load(&self, addr: u16) -> u8 {
        warn!("Load addr={:#x}", addr);
        0
    }

    /// store interface exposed to cpu via bus
    pub fn store(&mut self, addr: u16, val: u8) {
        match addr {
            0x00...0x03 => self.pulse1.store(addr, val),
            0x04...0x07 => self.pulse2.store(addr - 0x04, val),
            0x08...0x0b => self.triangle.store(addr - 0x08, val),
            0x0c...0x0f => self.noise.store(addr - 0x0c, val),
            0x15 => {
                self.pulse1.store(addr, val);
                self.pulse2.store(addr, val >> 1);
                self.triangle.store(addr, val >> 2);
                self.noise.store(addr, val >> 3);
            }
            0x17 => {
                self.pulse1.store(addr, val);
                self.pulse2.store(addr, val);
                self.triangle.store(addr, val);
                self.noise.store(addr, val);
            }
            0x11 => {}
            _ => {
                warn!("Store addr={:#x} val={:#x}", addr, val);
            }
        }
    }

    fn sample(&self) -> (f32, f32) {
        let p1 = self.pulse1.sample();
        let p2 = self.pulse2.sample();
        let t = self.triangle.sample();
        let n = self.noise.sample();

        let p_out = 95.88 / (((8128.0 / 15.0) / (p1 + p2)) + 100.0);
        let t_out = 159.79 / ((1.0 / ((t / (8227.0 / 15.0)) + (n / (12241.0 / 15.0)))) + 100.0);
        (p_out, t_out)
    }

    /// Tick 1 CPU clock
    pub fn tick(&mut self) {
        self.pulse1.tick();
        self.pulse2.tick();
        self.triangle.tick();
        self.noise.tick();

        if sample_index(self.clocks) != sample_index(self.clocks + 1) {
            self.append_buffer(self.sample());
        }

        self.clocks = (self.clocks + 1) % CPU_CLOCK_RATE;
    }

    fn append_buffer(&mut self, p: (f32, f32)) {
        if self.buffer.len() == BUFFER_LENGTH {
            self.buffer.pop_front();
        }
        self.buffer.push_back(p);
    }

    /// Get sampling buffer
    pub fn consume_buffer(&mut self) -> Drain<(f32, f32)> {
        self.buffer.drain(..)
    }
}
