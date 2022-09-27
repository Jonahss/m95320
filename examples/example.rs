use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use spi_memory::prelude::*;
use spi_memory::M95320::Flash;

const GPIO_MEMORY_CHIP_SELECT: u8 = 27;


fn main() {
    let gpio = Gpio::new().unwrap();
    let cs = gpio.get(GPIO_MEMORY_CHIP_SELECT).unwrap().into_output();
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 10_000_000, Mode::Mode0).unwrap();

    let mut flash = Flash::init(spi, cs).unwrap();

    let status = flash.read_status().expect("get status");
    println!("status registers: {:?}", status);

    let mut page_buffer: [u8; 32] = [0x0; 32];

    flash.erase_sectors(0, 2).expect("erase");

    let hello = String::from("hello memory!");
    for (i, byte) in hello.as_bytes().into_iter().enumerate() {
        page_buffer[i] = byte.clone();
    }

    flash.write_bytes(0, &mut page_buffer).expect("write");
    
    flash.read(0, &mut page_buffer).expect("read");
    println!("bytes read: {:?}", page_buffer);
}