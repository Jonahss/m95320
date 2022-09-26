use rppal::gpio::{ Gpio, Trigger };
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use spi_memory::prelude::*;
use spi_memory::M95320::Flash;

use std::time::Duration;
use std::thread;

const GPIO_MEMORY_CHIP_SELECT: u8 = 27;

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut cs = gpio.get(GPIO_MEMORY_CHIP_SELECT).unwrap().into_output();

    cs.set_high();

    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 10_000_000, Mode::Mode0).unwrap();

    let mut flash = Flash::init(spi, cs).unwrap();

    let status = flash.read_status().expect("get status");
    println!("status: {:?}", status);

    let mut page_buffer: [u8; 32] = [0x0; 32];
    let address: u16 = 30;

    flash.read(address, &mut page_buffer).expect("read");
    println!("read 32 bytes: {:?}", page_buffer);

    let hello = String::from("hello memory!");
    for (i, byte) in hello.as_bytes().into_iter().enumerate() {
        page_buffer[i] = byte.clone();
    }
    println!("write buffer before writing: {:?}", page_buffer);

    flash.write_bytes(0, &mut page_buffer).expect("write");

    let mut page_buffer: [u8; 32] = [0x0; 32];
    for (i, byte) in hello.as_bytes().into_iter().enumerate() {
        page_buffer[i] = byte.clone();
    } 

    flash.write_bytes(address, &mut page_buffer).expect("write");

    let status = flash.read_status().expect("get status");
    println!("status after write enable: {:?}", status);

    flash.read(0, &mut page_buffer).expect("read");
    println!("read first page of 32 bytes after write: {:?}", page_buffer);

    flash.read(32, &mut page_buffer).expect("read");
    println!("read second page of 32 bytes after write: {:?}", page_buffer);

    flash.erase_sectors(30, 2);

    flash.read(0, &mut page_buffer).expect("read");
    println!("read first page of 32 bytes after erase: {:?}", page_buffer);

    flash.read(32, &mut page_buffer).expect("read");
    println!("read second page of 32 bytes after erase: {:?}", page_buffer);

}