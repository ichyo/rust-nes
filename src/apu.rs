use log::trace;

const CPU_FREQ: f64 = 1.789_773 * 1_000_000.0;

#[derive(Default)]
/// audio processing unit. handles audio.
pub struct Apu {
    duty1: u8,
    volume1: u8,
    timer1: u16,
    duty2: u8,
    volume2: u8,
    timer2: u16,
    timer3: u16,
}

impl Apu {
    /// Create APU
    pub fn new() -> Apu {
        Apu {
            duty1: 0,
            volume1: 0,
            timer1: 0,
            volume2: 0,
            duty2: 0,
            timer2: 0,
            timer3: 0,
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
            0x00 => {
                self.volume1 = val & 0xf;
                self.duty1 = (val >> 6) & 0x3;
            }
            0x02 => {
                self.timer1 = (self.timer1 & 0x700) | (val as u16);
            }
            0x03 => {
                self.timer1 = (self.timer1 & 0xff) | ((val as u16 & 0x7) << 8);
            }
            0x04 => {
                self.volume2 = val & 0xf;
                self.duty2 = (val >> 6) & 0x3;
            }
            0x06 => {
                self.timer2 = (self.timer2 & 0x700) | (val as u16);
            }
            0x07 => {
                self.timer2 = (self.timer2 & 0xff) | ((val as u16 & 0x7) << 8);
            }
            0x0a => {
                self.timer3 = (self.timer3 & 0x700) | (val as u16);
            }
            0x0b => {
                self.timer3 = (self.timer3 & 0xff) | ((val as u16 & 0x7) << 8);
            }
            _ => {}
        }
    }

    /// TODO: remove this
    pub fn frequency1(&self) -> f64 {
        CPU_FREQ / (16.0 * (self.timer1 as f64 + 1.0))
    }

    /// TODO: remove this
    pub fn volume1(&self) -> f64 {
        self.volume1 as f64 / 15.0
    }

    /// TODO: remove this
    pub fn duty1(&self) -> f64 {
        match self.duty1 {
            0 => 0.125,
            1 => 0.25,
            2 => 0.5,
            3 => 0.75,
            _ => unreachable!(),
        }
    }

    /// TODO: remove this
    pub fn frequency2(&self) -> f64 {
        CPU_FREQ / (16.0 * (self.timer2 as f64 + 1.0))
    }

    /// TODO: remove this
    pub fn volume2(&self) -> f64 {
        self.volume2 as f64 / 15.0
    }

    /// TODO: remove this
    pub fn duty2(&self) -> f64 {
        match self.duty1 {
            0 => 0.125,
            1 => 0.25,
            2 => 0.5,
            3 => 0.75,
            _ => unreachable!(),
        }
    }

    /// TODO: remove this
    pub fn frequency3(&self) -> f64 {
        CPU_FREQ / (32.0 * (self.timer3 as f64 + 1.0))
    }
}
