pub struct PatternTable {
    memory: [u8; 0x2000],
}

impl PatternTable {
    pub fn new(chr_rom: &[u8]) -> PatternTable {
        assert_eq!(chr_rom.len(), 0x2000);
        let mut memory = [0; 0x2000];
        memory.clone_from_slice(chr_rom);
        PatternTable { memory }
    }

    pub fn load(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn get_value(&self, index: u16, x: u8, y: u8) -> u8 {
        assert!(x < 8);
        assert!(y < 8);
        let base = (index * 16) as usize;
        let c1 = self.memory[base + y as usize] >> (7 - x) & 1;
        let c2 = self.memory[base + y as usize + 8] >> (7 - x) & 1;
        (c2 << 1) | c1
    }

    pub fn get_left_value(&self, index: u8, x: u8, y: u8) -> u8 {
        self.get_value(u16::from(index), x, y)
    }

    pub fn get_right_value(&self, index: u8, x: u8, y: u8) -> u8 {
        self.get_value(u16::from(index) + 0x100, x, y)
    }
}
