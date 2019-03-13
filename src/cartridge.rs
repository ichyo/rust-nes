use std::error::Error;

const PRG_ROM_PAGE_UNIT: usize = 16 * 1024;
const CHR_ROM_PAGE_UNIT: usize = 8 * 1024;

pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

fn read_byte(iter: &mut Iterator<Item = u8>) -> Result<u8, Box<Error>> {
    match iter.next() {
        Some(val) => Ok(val),
        None => Err("unexpected end of file".into()),
    }
}

fn read_bytes(iter: &mut Iterator<Item = u8>, bytes: usize) -> Result<Vec<u8>, Box<Error>> {
    let buf: Vec<_> = iter.take(bytes).collect();
    if buf.len() < bytes {
        Err("unexpected end of file".into())
    } else {
        Ok(buf)
    }
}

impl Cartridge {
    pub fn parse(d: &[u8]) -> Result<Cartridge, Box<Error>> {
        let mut it = d.iter().cloned();
        let magic = read_bytes(&mut it, 4)?;
        if magic != b"NES\x1a" {
            return Err("damaged header".into());
        }
        let prg_rom_pages = read_byte(&mut it)? as usize;
        let chr_rom_pages = read_byte(&mut it)? as usize;
        let _ = read_bytes(&mut it, 10)?;
        let prg_rom = read_bytes(&mut it, prg_rom_pages * PRG_ROM_PAGE_UNIT)?;
        let chr_rom = read_bytes(&mut it, chr_rom_pages * CHR_ROM_PAGE_UNIT)?;
        Ok(Cartridge { prg_rom, chr_rom })
    }
}
