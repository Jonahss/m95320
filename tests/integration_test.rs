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
        assert_eq!(page_buffer, [104, 101, 108, 108, 111, 32, 109, 101, 109, 111, 114, 121, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let mut page_buffer: [u8; 32] = [0x0; 32];
        for (i, byte) in hello.as_bytes().into_iter().enumerate() {
            page_buffer[i] = byte.clone();
        } 

        flash.write_bytes(30, &mut page_buffer).expect("write");
        flash.read(0, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [104, 101, 108, 108, 111, 32, 109, 101, 109, 111, 114, 121, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 101]);

        flash.read(32, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [108, 108, 111, 32, 109, 101, 109, 111, 114, 121, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        flash.read(5, &mut page_buffer).expect("read");
        assert_eq!(page_buffer, [32, 109, 101, 109, 111, 114, 121, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 101, 108, 108, 111, 32, 109]);
    }
}