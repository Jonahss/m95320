use crate::{ BlockDevice, Error, Read };

use bitflags::bitflags;

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

const PAGE_SIZE: u16 = 32;
enum Opcode {
    WriteEnable = 0x06,
    #[allow(dead_code)]
    WriteDisable = 0x04,
    ReadStatusRegister = 0x05,
    #[allow(dead_code)]
    WriteStatusRegister = 0x01,
    Read = 0x03,
    Write = 0x02,
}

bitflags! {
    /// Status register bits.
    pub struct Status: u8 {
        /// Erase or write in progress.
        const WRITE_IN_PROGRESS = 1 << 0;
        /// Status of the **W**rite **E**nable **L**atch.
        const WRITE_ENABLE_LATCH = 1 << 1;
        /// The 2 protection region bits.
        const BLOCK_PROTECT = 0b00001100;
        /// **S**tatus **R**egister **W**rite **D**isable bit.
        const STATUS_REGISTER_WRITE_DISABLE = 1 << 7;
    }
}

/// Driver for M95320 SPI Flash chips.
///
/// Implementation is not complete. Missing ability to write to status registers,
/// write-protect, hold functionality, and soft reset
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
    pub fn init(spi: SPI, cs: CS) -> Result<Self, Error<SPI, CS>> {
        let mut this = Self { spi, cs };
        this.cs.set_high().map_err(Error::Gpio)?;
        let status = this.read_status()?;
        info!("Flash::init: status = {:?}", status);

        // Here we don't expect any writes to be in progress, and the latch must
        // also be deasserted.
        if !(status & (Status::WRITE_IN_PROGRESS | Status::WRITE_ENABLE_LATCH)).is_empty() {
            return Err(Error::UnexpectedStatus);
        }

        Ok(this)
    }

    fn command(&mut self, bytes: &mut [u8]) -> Result<(), Error<SPI, CS>> {
        // If the SPI transfer fails, make sure to disable CS anyways
        self.cs.set_low().map_err(Error::Gpio)?;
        let spi_result = self.spi.transfer(bytes).map_err(Error::Spi);
        self.cs.set_high().map_err(Error::Gpio)?;
        spi_result?;
        Ok(())
    }

    /// Reads the status register.
    pub fn read_status(&mut self) -> Result<Status, Error<SPI, CS>> {
        let mut buf = [Opcode::ReadStatusRegister as u8, 0];
        self.command(&mut buf)?;

        Ok(Status::from_bits_truncate(buf[1]))
    }

    fn write_enable(&mut self) -> Result<(), Error<SPI, CS>> {
        let mut cmd_buf = [Opcode::WriteEnable as u8];
        self.command(&mut cmd_buf)?;
        Ok(())
    }

    fn wait_done(&mut self) -> Result<(), Error<SPI, CS>> {
        // TODO: Consider changing this to a delay based pattern
        while self.read_status()?.contains(Status::WRITE_IN_PROGRESS) {}
        Ok(())
    }

    fn write_bytes_to_page(&mut self, addr: u16, data: &mut [u8]) -> Result<(), Error<SPI, CS>> {
        if addr > 2u16.pow(12)-1 {
            return Err(Error::AddressOutOfBounds(addr.into()))
        }

        self.write_enable()?;

        let mut cmd_buf = [
            Opcode::Write as u8,
            (addr >> 8) as u8,
            addr as u8,
        ];

        self.cs.set_low().map_err(Error::Gpio)?;
        let mut spi_result = self.spi.transfer(&mut cmd_buf);
        if spi_result.is_ok() {
            spi_result = self.spi.transfer(data);
        }
        self.cs.set_high().map_err(Error::Gpio)?;
        spi_result.map(|_| ()).map_err(Error::Spi)?;

        self.wait_done()?;
        Ok(())
    }
}

impl<SPI: Transfer<u8>, CS: OutputPin> Read<u16, SPI, CS> for Flash<SPI, CS> {
    /// # Parameters
    ///
    /// * `addr`: 16-bit address to start reading at.
    /// * `buf`: Destination buffer to fill.
    fn read(&mut self, addr: u16, buf: &mut [u8]) -> Result<(), Error<SPI, CS>> {
        // TODO what happens if `buf` is empty?

        let mut cmd_buf = [
            Opcode::Read as u8,
            (addr >> 8) as u8,
            addr as u8,
        ];

        self.cs.set_low().map_err(Error::Gpio)?;
        let mut spi_result = self.spi.transfer(&mut cmd_buf);
        if spi_result.is_ok() {
            spi_result = self.spi.transfer(buf);
        }
        self.cs.set_high().map_err(Error::Gpio)?;
        spi_result.map(|_| ()).map_err(Error::Spi)
    }
}

impl<SPI: Transfer<u8>, CS: OutputPin> BlockDevice<u16, SPI, CS> for Flash<SPI, CS> {
    /// # Parameters
    /// 
    /// * `addr`: address to start erasing at
    /// * `amount`: number of 32byte pages to erase, including the first partial page
    fn erase_sectors(&mut self, addr: u16, amount: usize) -> Result<(), Error<SPI, CS>> {
        let first_chunk_length = PAGE_SIZE - (addr % PAGE_SIZE);
        let mut buf = [0; 32];

        self.write_bytes(addr, &mut buf[..first_chunk_length.into()])?;

        let mut current_addr = addr - first_chunk_length + PAGE_SIZE;
        for _ in 1..amount {
            let mut buf = [0; 32];
            self.write_bytes(current_addr, &mut buf)?;
            current_addr += PAGE_SIZE;
        }
    
        Ok(())
    }

    fn write_bytes(&mut self, addr: u16, data: &mut [u8]) -> Result<(), Error<SPI, CS>> {
        let mut current_addr: u16 = addr;

        let first_chunk_length = PAGE_SIZE - (current_addr % PAGE_SIZE);
        let (first_chunk, rest_of_data) = data.split_at_mut(first_chunk_length.into());

        self.write_bytes_to_page(addr, first_chunk)?;

        // align address to page boundary
        let remainder = current_addr % PAGE_SIZE;
        current_addr = current_addr - remainder + PAGE_SIZE;

        for chunk_data in rest_of_data.rchunks_mut(PAGE_SIZE.into()).rev() {
            self.write_bytes_to_page(current_addr, chunk_data)?;
    
            // advance to next page
            current_addr += PAGE_SIZE;
        }
   
        Ok(())
    }

    fn erase_all(&mut self) -> Result<(), Error<SPI, CS>> {
        self.erase_sectors(0, (PAGE_SIZE/125).into())?;

        Ok(())
    }
}