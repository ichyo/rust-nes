use log::trace;

#[derive(Default)]
/// audio processing unit. handles audio.
pub struct Apu {}

impl Apu {
    /// Create APU
    pub fn new() -> Apu {
        Apu {}
    }

    /// load interface exposed to cpu via bus
    pub fn load(&self, addr: u16) -> u8 {
        trace!("Load addr={:#x}", addr);
        0
    }

    /// store interface exposed to cpu via bus
    pub fn store(&mut self, addr: u16, val: u8) {
        trace!("Store addr={:#x} val={:#x}", addr, val);
    }
}
