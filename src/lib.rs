#[macro_use]
extern crate lazy_static;

pub mod cartridge;
pub mod cpu;
pub mod memory;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
