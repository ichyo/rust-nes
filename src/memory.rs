const MEMORY_LENGTH: usize = 2 * 1024;

#[derive(Clone)]
/// volatile memory in NES. 2kB
pub struct Memory([u8; MEMORY_LENGTH]);
impl Memory {
    /// Create memory filled with zeros.
    pub fn new() -> Memory {
        Memory([0; MEMORY_LENGTH])
    }

    /// Load 8bit value from address
    pub fn load(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    /// Store 8bit value into address
    pub fn store(&mut self, addr: u16, val: u8) {
        self.0[addr as usize] = val;
    }
}

impl Default for Memory {
    fn default() -> Memory {
        Memory::new()
    }
}
