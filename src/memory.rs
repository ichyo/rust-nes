pub trait Memory {
    fn load(&self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, val: u8);

    fn load_w(&self, addr: u16) -> u16 {
        self.load(addr) as u16 | ((self.load(addr + 1) as u16) << 8)
    }
    fn store_w(&mut self, addr: u16, val: u16) {
        self.store(addr, (val & 0xff) as u8);
        self.store(addr + 1, (val >> 8) as u8);
    }
}
