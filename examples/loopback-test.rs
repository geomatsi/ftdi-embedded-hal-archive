#![feature(extern_crate_item_prelude)]

#[macro_use]
extern crate itertools;
extern crate ftdi_embedded_hal as hal;
extern crate rand;

use embedded_hal::blocking::spi::Transfer;
use hal::ft232h::FT232H;
use rand::Rng;

fn main() {
    let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
    dev.loopback(true).unwrap();

    let mut spidev = dev.spi().unwrap();

    // loopback: 1-byte messages
    for v in 0x0..0xff {
        let mut tx = [v; 1];
        let cx = tx.clone();
        let rx = spidev.transfer(&mut tx).unwrap();

        assert_eq!(cx, rx);
    }

    // loopback: 3-byte messages
    for (x, y, z) in iproduct!(1..5, 11..15, 21..25) {
        let mut tx = [x, y, z];
        let cx = tx.clone();
        let rx = spidev.transfer(&mut tx).unwrap();
        assert_eq!(cx, rx);
    }

    // loopback: 5-byte random messages
    for _ in 1..10 {
        let mut rng = rand::thread_rng();
        let mut tx: Vec<u8> = (0..5)
            .map(|_| {
                // 0 (inclusive) to 254 (inclusive)
                rng.gen_range(0, 255)
            })
            .collect();
        let cx = tx.clone();
        let rx = spidev.transfer(&mut tx).unwrap();
        assert_eq!(cx, rx);
    }

    println!("Loopback ok!");
}
