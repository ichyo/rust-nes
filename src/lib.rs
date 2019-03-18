#![deny(clippy::all)]
#![deny(missing_docs)]

//! NES emulator written in rust

/// audio processing uni
pub mod apu;
/// memory map for cpu
pub mod bus;
/// cartridge with two memory units
pub mod cartridge;
/// processor
pub mod cpu;
/// volatile memory
pub mod memory;
/// picture processing unit
pub mod ppu;
