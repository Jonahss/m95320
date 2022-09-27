//! [`embedded-hal`] Rust driver for STMicroelectronics M95320 32-Kbit serial SPI bus EEPROM
//! 
//! *some features not yet implemented, basic read and write is working*
//! 
// ! This create is mostly ripped-off from the `spi-memory` crate: https://github.com/jonas-schievink/spi-memory

#![doc(html_root_url = "https://docs.rs/m95320/1.0.0")]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![cfg_attr(not(test), no_std)]

#[macro_use]
mod log;
mod error;
pub mod prelude;
pub mod m95320;
mod utils;

pub use crate::error::Error;

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// A trait for reading operations from a memory chip.
pub trait Read<Addr, SPI: Transfer<u8>, CS: OutputPin> {
    /// Reads bytes from a memory chip.
    ///
    /// # Parameters
    /// * `addr`: The address to start reading at.
    /// * `buf`: The buffer to read `buf.len()` bytes into.
    fn read(&mut self, addr: Addr, buf: &mut [u8]) -> Result<(), Error<SPI, CS>>;
}

/// A trait for writing and erasing operations on a memory chip.
pub trait BlockDevice<Addr, SPI: Transfer<u8>, CS: OutputPin> {
    /// Erases sectors from the memory chip.
    ///
    /// # Parameters
    /// * `addr`: The address to start erasing at. If the address is not on a sector boundary,
    ///   the lower bits can be ignored in order to make it fit.
    fn erase_sectors(&mut self, addr: Addr, amount: usize) -> Result<(), Error<SPI, CS>>;

    /// Erases the memory chip fully.
    ///
    /// Warning: Full erase operations can take a significant amount of time.
    /// Check your device's datasheet for precise numbers.
    fn erase_all(&mut self) -> Result<(), Error<SPI, CS>>;

    /// Writes bytes onto the memory chip. This method is supposed to assume that the sectors
    /// it is writing to have already been erased and should not do any erasing themselves.
    ///
    /// # Parameters
    /// * `addr`: The address to write to.
    /// * `data`: The bytes to write to `addr`.
    fn write_bytes(&mut self, addr: Addr, data: &mut [u8]) -> Result<(), Error<SPI, CS>>;
}
