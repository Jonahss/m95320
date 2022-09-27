# `m95320`

[![crates.io](https://img.shields.io/crates/v/m95320.svg)](https://crates.io/crates/m95320)
[![docs.rs](https://docs.rs/m95320/badge.svg)](https://docs.rs/m95320/)

[`embedded-hal`] Rust driver for STMicroelectronics M95320 32-Kbit serial SPI bus EEPROM

*some features not yet implemented, basic read and write is working*

This create is mostly ripped-off from the `spi-memory` crate: https://github.com/jonas-schievink/spi-memory


[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

## Usage

Add an entry to your `Cargo.toml`:

```toml
[dependencies]
m95320 = "1.0.1"
```

## Example
Using `rppal` on a Raspberry Pi:
```
use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use m95320::prelude::*;
use m95320::m95320::Flash;

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
```

Check the [API Documentation](https://docs.rs/m95320/) for how to use the
crate's functionality.
