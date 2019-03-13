use log::trace;

#[derive(Default)]
pub struct Apu {}

impl Apu {
    pub fn new() -> Apu {
        Apu {}
    }
    pub fn load(&self, addr: u16) -> u8 {
        trace!("Load addr={:#x}", addr);
        0
    }
    pub fn store(&mut self, addr: u16, val: u8) {
        trace!("Store addr={:#x} val={:#x}", addr, val);
    }
}
