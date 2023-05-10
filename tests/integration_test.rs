/// These tests use a Raspberry Pi connected to the memory chip
/// and the `rppal` raspeberry pi embedded-hal library

use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use m95320::prelude::*;
use m95320::m95320::Flash;

const GPIO_MEMORY_CHIP_SELECT: u8 = 27;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let gpio = Gpio::new().unwrap();
        let cs = gpio.get(GPIO_MEMORY_CHIP_SELECT).unwrap().into_output();
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 10_000_000, Mode::Mode0).unwrap();

        let mut flash = Flash::init(spi, cs).unwrap();

        let _status = flash.read_status().expect("get status");

        let mut page_buffer: [u8; 32] = [0x0; 32];

        flash.erase_sectors(0, 2).expect("erase");


        flash.read(0, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        flash.read(5, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        flash.read(32, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let hello = String::from("hello memory!");
        for (i, byte) in hello.as_bytes().into_iter().enumerate() {
            page_buffer[i] = byte.clone();
        }

        flash.write_bytes(0, &mut page_buffer).expect("write");
        flash.read(0, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [104, 101, 108, 108, 111, 32, 109, 101, 109, 111, 114, 121, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "simple write and read of first page");

        let mut page_buffer: [u8; 32] = [0x0; 32];
        for (i, byte) in hello.as_bytes().into_iter().enumerate() {
            page_buffer[i] = byte.clone();
        } 

        flash.write_bytes(30, &mut page_buffer).expect("write");
        flash.read(0, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [104, 101, 108, 108, 111, 32, 109, 101, 109, 111, 114, 121, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 101], "write at address straddling page 0 and page 1, read page 1");

        flash.read(32, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [108, 108, 111, 32, 109, 101, 109, 111, 114, 121, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "write at address straddling page 0 and page 1, read page 2");

        flash.read(5, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [32, 109, 101, 109, 111, 114, 121, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 101, 108, 108, 111, 32, 109], "read straddling page boundary");
    
        let chuckwudi = String::from("chuckwudi");
        let mut small_buffer: [u8; 9] = [0x0; 9];
        for (i, byte) in chuckwudi.as_bytes().into_iter().enumerate() {
            small_buffer[i] = byte.clone();
        }
        flash.write_bytes(0, &mut small_buffer).expect("write");
        flash.read(0, &mut small_buffer).expect("read");
        assert_eq!(small_buffer, [0x63, 0x68, 0x75, 0x63, 0x6b, 0x77, 0x75, 0x64, 0x69], "write and read a buffer smaller than a page");

        flash.read(1, &mut small_buffer).expect("read");
        assert_eq!(small_buffer, [0x68, 0x75, 0x63, 0x6b, 0x77, 0x75, 0x64, 0x69, 111], "write and read a buffer smaller than a page, offset 1");


        let mut small_buffer: [u8; 9] = [0x0; 9];
        for (i, byte) in chuckwudi.as_bytes().into_iter().enumerate() {
            small_buffer[i] = byte.clone();
        }
        flash.write_bytes(30, &mut small_buffer).expect("write");
        flash.read(30, &mut small_buffer).expect("read");
        assert_eq!(small_buffer, [0x63, 0x68, 0x75, 0x63, 0x6b, 0x77, 0x75, 0x64, 0x69], "write and read a buffer smaller than a page at page boundary");
    }
}