pub struct Ppu {}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {}
    }
    pub fn load(&self, addr: u16) -> u8 {
        eprintln!("[Ppu] load addr={:#x}", addr);
        0
    }
    pub fn store(&mut self, addr: u16, val: u8) {
        eprintln!("[Ppu] store addr={:#x} val={:#x}", addr, val);
    }
}
