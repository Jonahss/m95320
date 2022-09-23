use crate::{utils::HexSlice, BlockDevice, Error, Read};

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;


/// Driver for M95320 SPI Flash chips.
///
/// # Type Parameters
///
/// * **`SPI`**: The SPI master to which the flash chip is attached.
/// * **`CS`**: The **C**hip-**S**elect line attached to the `\CS`/`\CE` pin of
///   the flash chip.
#[derive(Debug)]
pub struct Flash<SPI: Transfer<u8>, CS: OutputPin> {
    spi: SPI,
    cs: CS,
}

impl<SPI: Transfer<u8>, CS: OutputPin> Flash<SPI, CS> {

}

impl<SPI: Transfer<u8>, CS: OutputPin> Read<u32, SPI, CS> for Flash<SPI, CS> {
    fn read(&mut self, addr: u32, buf: &mut [u8]) -> Result<(), Error<SPI, CS>> {
        Ok(())
    }
}