#![feature(extern_crate_item_prelude)]

extern crate embedded_hal as hal;
extern crate ftdi;
extern crate nb;

extern crate itertools;
extern crate rand;

pub mod ft232h;
pub mod gpio;
pub mod i2c;
pub mod mpsse;
pub mod spi;

//

#[cfg(test)]
mod test {
    use super::ft232h::FT232H;
    use embedded_hal::blocking::spi::Transfer;
    use itertools::iproduct;
    use rand::Rng;
    use std::io::Error;

    #[test]
    fn test_init_t1() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        assert_eq!(dev.is_loopback(), false);

        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let spidev = dev.spi().unwrap();
        assert_eq!(spidev.get_speed(), 0);
    }

    #[test]
    fn test_init_t2() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        assert_eq!(dev.is_loopback(), false);

        let ph0 = dev.ph0().unwrap();
        assert_eq!(ph0.get_bit(), 0b0000_0001);
    }

    #[test]
    fn test_init_t3() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        assert_eq!(dev.is_loopback(), false);

        let ph0_0 = dev.ph0();
        let ph0_1 = dev.ph0();
        let ph0_2 = dev.ph0();

        assert!(ph0_0.is_ok(), "First pin instance should be OK");
        assert!(ph0_1.is_err(), "There should be no second pin instance");
        assert!(ph0_2.is_err(), "There should be no third pin instance");
    }

    #[test]
    fn test_loopback_t1() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let mut spidev = dev.spi().unwrap();

        // loopback: 1-byte messages
        for v in 0x0..0xff {
            let mut tx = [v; 1];
            let cx = tx.clone();
            let rx = spidev.transfer(&mut tx).unwrap();

            assert_eq!(cx, rx);
        }
    }

    #[test]
    fn test_loopback_t2() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let mut spidev = dev.spi().unwrap();

        // loopback: 3-byte messages
        for (x, y, z) in iproduct!(1..5, 11..15, 21..25) {
            let mut tx = [x, y, z];
            let cx = tx.clone();
            let rx = spidev.transfer(&mut tx).unwrap();
            assert_eq!(cx, rx);
        }
    }

    #[test]
    fn test_loopback_t3() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let mut spidev = dev.spi().unwrap();

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
    }

    #[test]
    fn test_loopback_multi_bus_t1() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let mut spidev1 = dev.spi().unwrap();
        let mut spidev2 = dev.spi().unwrap();

        // loopback: 1-byte messages on both protocol buses
        for v in 0x0..0xff {
            let mut tx = [v; 1];
            let cx = tx.clone();

            let rx = spidev1.transfer(&mut tx).unwrap();
            assert_eq!(cx, rx);

            let rx = spidev2.transfer(&mut tx).unwrap();
            assert_eq!(cx, rx);
        }
    }
}
