pub struct PatternTables {
    left: PatternTable,
    right: PatternTable,
}

const PATTERN_TABLE_LENGTH: usize = 0x1000;

pub struct PatternTable {
    memory: [u8; PATTERN_TABLE_LENGTH],
}

pub enum PatternTableSide {
    Left,
    Right,
}

impl PatternTableSide {
    fn from_addr(addr: u16) -> PatternTableSide {
        let id = addr / PATTERN_TABLE_LENGTH as u16;
        match id {
            0 => PatternTableSide::Left,
            1 => PatternTableSide::Right,
            _ => unreachable!(),
        }
    }
}

impl PatternTables {
    pub fn new(chr_rom: &[u8]) -> Self {
        assert_eq!(chr_rom.len(), 0x2000);
        PatternTables {
            left: PatternTable::new(&chr_rom[0..0x1000]),
            right: PatternTable::new(&chr_rom[0x1000..0x2000]),
        }
    }

    pub fn load(&self, addr: u16) -> u8 {
        self.get_table(PatternTableSide::from_addr(addr))
            .load(addr % PATTERN_TABLE_LENGTH as u16)
    }

    pub fn get_table(&self, side: PatternTableSide) -> &PatternTable {
        match side {
            PatternTableSide::Left => &self.left,
            PatternTableSide::Right => &self.right,
        }
    }
}

impl PatternTable {
    pub fn new(d: &[u8]) -> PatternTable {
        assert_eq!(d.len(), 0x1000);
        let mut memory = [0; 0x1000];
        memory.clone_from_slice(d);
        PatternTable { memory }
    }

    pub fn load(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn get_value(&self, index: u8, x: u8, y: u8) -> u8 {
        assert!(x < 8);
        assert!(y < 8);
        let base = index as usize * 16;
        let c1 = self.memory[base + y as usize] >> (7 - x) & 1;
        let c2 = self.memory[base + y as usize + 8] >> (7 - x) & 1;
        (c2 << 1) | c1
    }
}
