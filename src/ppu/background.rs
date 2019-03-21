const TABLE_SIZE: u16 = 0x400;
const NAME_TABLE_SIZE: u16 = 0x3c0;

const TABLE_WIDTH: u16 = 256;
const TABLE_HEIGHT: u16 = 240;
const BLOCK_WIDTH: u16 = 8;
const BLOCK_HEIGHT: u16 = 8;
const ATTRIBUTE_BLOCK_WIDTH: u16 = 32;
const ATTRIBUTE_BLOCK_HEIGHT: u16 = 32;

#[derive(Clone, Copy)]
enum MirroringMode {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy)]
enum Screen {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Screen {
    fn from_addr(addr: u16) -> Screen {
        let id = (addr % 0x1000) / 0x400;
        match id {
            0 => Screen::TopLeft,
            1 => Screen::TopRight,
            2 => Screen::BottomLeft,
            3 => Screen::BottomRight,
            _ => unreachable!(),
        }
    }

    fn from_coord(x: u16, y: u16) -> Screen {
        match (x < TABLE_WIDTH, y < TABLE_HEIGHT) {
            (true, true) => Screen::TopLeft,
            (false, true) => Screen::TopRight,
            (true, false) => Screen::BottomLeft,
            (false, false) => Screen::BottomRight,
        }
    }
}

pub struct NameTables {
    table1: NameTable,
    table2: NameTable,
    mirroing: MirroringMode,
}

impl NameTables {
    pub fn new() -> NameTables {
        NameTables {
            table1: NameTable::new(),
            table2: NameTable::new(),
            mirroing: MirroringMode::Vertical,
        }
    }

    pub fn get_pattern_index(&self, x: u16, y: u16) -> u8 {
        self.get_table(Screen::from_coord(x, y))
            .get_pattern_index(x % TABLE_WIDTH, y % TABLE_HEIGHT)
    }

    pub fn get_palette_index(&self, x: u16, y: u16) -> u8 {
        self.get_table(Screen::from_coord(x, y))
            .get_palette_index(x % TABLE_WIDTH, y % TABLE_HEIGHT)
    }

    pub fn set_varical_mirroring(&mut self) {
        self.mirroing = MirroringMode::Vertical;
    }

    pub fn set_horizontal_mirroring(&mut self) {
        self.mirroing = MirroringMode::Horizontal;
    }

    fn get_table(&self, screen: Screen) -> &NameTable {
        match (self.mirroing, screen) {
            (MirroringMode::Vertical, Screen::TopLeft) => &self.table1,
            (MirroringMode::Vertical, Screen::TopRight) => &self.table2,
            (MirroringMode::Vertical, Screen::BottomLeft) => &self.table1,
            (MirroringMode::Vertical, Screen::BottomRight) => &self.table2,
            (MirroringMode::Horizontal, Screen::TopLeft) => &self.table1,
            (MirroringMode::Horizontal, Screen::TopRight) => &self.table1,
            (MirroringMode::Horizontal, Screen::BottomLeft) => &self.table2,
            (MirroringMode::Horizontal, Screen::BottomRight) => &self.table2,
        }
    }

    fn get_table_mut(&mut self, screen: Screen) -> &mut NameTable {
        match (self.mirroing, screen) {
            (MirroringMode::Vertical, Screen::TopLeft) => &mut self.table1,
            (MirroringMode::Vertical, Screen::TopRight) => &mut self.table2,
            (MirroringMode::Vertical, Screen::BottomLeft) => &mut self.table1,
            (MirroringMode::Vertical, Screen::BottomRight) => &mut self.table2,
            (MirroringMode::Horizontal, Screen::TopLeft) => &mut self.table1,
            (MirroringMode::Horizontal, Screen::TopRight) => &mut self.table1,
            (MirroringMode::Horizontal, Screen::BottomLeft) => &mut self.table2,
            (MirroringMode::Horizontal, Screen::BottomRight) => &mut self.table2,
        }
    }

    pub fn load(&self, addr: u16) -> u8 {
        self.get_table(Screen::from_addr(addr))
            .load(addr % TABLE_SIZE)
    }

    pub fn store(&mut self, addr: u16, value: u8) {
        self.get_table_mut(Screen::from_addr(addr))
            .store(addr % TABLE_SIZE, value)
    }
}

struct NameTable {
    memory: [u8; TABLE_SIZE as usize],
}

impl NameTable {
    fn new() -> NameTable {
        NameTable {
            memory: [0; TABLE_SIZE as usize],
        }
    }

    fn get_pattern_index(&self, x: u16, y: u16) -> u8 {
        let block_x = x / BLOCK_WIDTH;
        let block_y = y / BLOCK_HEIGHT;
        let addr = block_x + block_y * (TABLE_WIDTH / BLOCK_WIDTH);
        self.memory[addr as usize]
    }

    fn get_palette_index(&self, x: u16, y: u16) -> u8 {
        let attr_x = x / ATTRIBUTE_BLOCK_WIDTH;
        let attr_y = y / ATTRIBUTE_BLOCK_HEIGHT;
        let attr_index = attr_x + attr_y * (TABLE_WIDTH / ATTRIBUTE_BLOCK_WIDTH);
        let addr = NAME_TABLE_SIZE + attr_index;
        let shift = match (
            x < ATTRIBUTE_BLOCK_WIDTH / 2,
            y < ATTRIBUTE_BLOCK_HEIGHT / 2,
        ) {
            (true, true) => 0,
            (false, true) => 2,
            (true, false) => 4,
            (false, false) => 6,
        };
        (self.memory[addr as usize] >> shift) % 4
    }

    fn load(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn store(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value
    }
}
