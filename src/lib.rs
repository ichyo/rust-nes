#![deny(clippy::all)]
#![allow(clippy::if_same_then_else)] // needs this until ppu is implemented
pub mod apu;
pub mod cartridge;
pub mod cpu;
pub mod memory;
pub mod ppu;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
