const MEMORY_LENGTH: usize = 2 * 1024;

pub struct Memory([u8; MEMORY_LENGTH]);
impl Memory {
    pub fn new() -> Memory {
        Memory([0; MEMORY_LENGTH])
    }

    pub fn load(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn store(&mut self, addr: u16, val: u8) {
        self.0[addr as usize] = val;
    }
}
