#![feature(extern_crate_item_prelude)]

#[macro_use]
extern crate itertools;
extern crate ftdi_embedded_hal as hal;
extern crate rand;

use embedded_hal::blocking::spi::Transfer;
use hal::devices::FtdiDevice;
use rand::Rng;

fn main() {
    let mut dev = FtdiDevice::spi_init(0x0403, 0x6014, None).unwrap();

    dev.loopback(true).unwrap();

    // loopback: 1-byte messages
    for v in 0x0..0xff {
        let mut tx = [v; 1];
        let cx = tx.clone();
        let rx = dev.transfer(&mut tx).unwrap();

        assert_eq!(cx, rx);
    }

    // loopback: 3-byte messages
    for (x, y, z) in iproduct!(1..5, 11..15, 21..25) {
        let mut tx = [x, y, z];
        let cx = tx.clone();
        let rx = dev.transfer(&mut tx).unwrap();
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
        let rx = dev.transfer(&mut tx).unwrap();
        assert_eq!(cx, rx);
    }

    println!("Loopback ok!");
}
