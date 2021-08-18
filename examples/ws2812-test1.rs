extern crate ftdi_embedded_hal as hal;

use crate::hal::x232h::FTx232H;

use embedded_hal::blocking::spi::Write;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let dev = FTx232H::init(0x0403, 0x6014).unwrap();
    let mut spi = dev.spi(hal::spi::SpiSpeed::CLK_3MHz).unwrap();

    // spi sequence for ws2812 color value 0x10
    let b1 = [0x92, 0x69, 0x24];

    // spi sequence for ws2812 color value 0x00
    let b0 = [0x92, 0x49, 0x24];

    // spi sequences for single led of specific color
    let g = [b1, b0, b0];
    let r = [b0, b1, b0];
    let b = [b0, b0, b1];
    let x = [b0, b0, b0];

    // initial pattern
    let mut pattern = vec![r, r, g, g, x, x, b, b];

    println!("ready to go...");

    loop {
        println!("next pattern...");
        let stream = pattern
            .clone()
            .into_iter()
            .flatten()
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>();

        spi.write(stream.as_slice()).unwrap();
        sleep(Duration::from_millis(400));
        // rotate pattern
        pattern.rotate_right(1);
    }
}
