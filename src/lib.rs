extern crate embedded_hal;
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
    use crate::gpio::PinBank;
    use embedded_hal::blocking::spi::Transfer;
    use itertools::iproduct;
    use rand::Rng;

    #[test]
    fn ft232h_test_init_t1() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        assert_eq!(dev.is_loopback(), false);

        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let mut spidev = dev.spi().unwrap();
        assert_eq!(spidev.get_speed(), 0);
        spidev.set_speed(100);
        assert_eq!(spidev.get_speed(), 100);
        spidev.set_speed(1000);
        assert_eq!(spidev.get_speed(), 1000);
    }

    #[test]
    fn ft232h_test_init_t2() {
        let dev = FT232H::init(0x0403, 0x6014).unwrap();

        let pl0 = dev.pl0().unwrap();
        assert_eq!(pl0.get_bit(), 0b0001_0000);
        assert_eq!(pl0.get_bank(), PinBank::Low);

        let pl1 = dev.pl1().unwrap();
        assert_eq!(pl1.get_bit(), 0b0010_0000);
        assert_eq!(pl1.get_bank(), PinBank::Low);

        let pl2 = dev.pl2().unwrap();
        assert_eq!(pl2.get_bit(), 0b0100_0000);
        assert_eq!(pl2.get_bank(), PinBank::Low);

        let pl3 = dev.pl3().unwrap();
        assert_eq!(pl3.get_bit(), 0b1000_0000);
        assert_eq!(pl3.get_bank(), PinBank::Low);
    }

    #[test]
    fn ft232h_test_init_t3() {
        let dev = FT232H::init(0x0403, 0x6014).unwrap();

        let ph0 = dev.ph0().unwrap();
        assert_eq!(ph0.get_bit(), 0b0000_0001);
        assert_eq!(ph0.get_bank(), PinBank::High);

        let ph1 = dev.ph1().unwrap();
        assert_eq!(ph1.get_bit(), 0b0000_0010);
        assert_eq!(ph1.get_bank(), PinBank::High);

        let ph2 = dev.ph2().unwrap();
        assert_eq!(ph2.get_bit(), 0b0000_0100);
        assert_eq!(ph2.get_bank(), PinBank::High);

        let ph3 = dev.ph3().unwrap();
        assert_eq!(ph3.get_bit(), 0b0000_1000);
        assert_eq!(ph3.get_bank(), PinBank::High);

        let ph4 = dev.ph4().unwrap();
        assert_eq!(ph4.get_bit(), 0b0001_0000);
        assert_eq!(ph4.get_bank(), PinBank::High);

        let ph5 = dev.ph5().unwrap();
        assert_eq!(ph5.get_bit(), 0b0010_0000);
        assert_eq!(ph5.get_bank(), PinBank::High);

        let ph6 = dev.ph6().unwrap();
        assert_eq!(ph6.get_bit(), 0b0100_0000);
        assert_eq!(ph6.get_bank(), PinBank::High);

        let ph7 = dev.ph7().unwrap();
        assert_eq!(ph7.get_bit(), 0b1000_0000);
        assert_eq!(ph7.get_bank(), PinBank::High);
    }

    #[test]
    fn ft232h_test_init_t4() {
        let dev = FT232H::init(0x0403, 0x6014).unwrap();
        assert_eq!(dev.is_loopback(), false);

        let ph0_0 = dev.ph0();
        let ph0_1 = dev.ph0();
        let ph0_2 = dev.ph0();

        assert!(ph0_0.is_ok(), "First pin instance should be OK");
        assert!(ph0_1.is_err(), "There should be no second pin instance");
        assert!(ph0_2.is_err(), "There should be no third pin instance");
    }

    #[test]
    fn ft232h_test_loopback_t1() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let mut spidev = dev.spi().unwrap();

        // loopback: 1-byte messages
        for v in 0x0..0xff {
            let mut tx = [v; 1];
            let cx = tx;
            let rx = spidev.transfer(&mut tx).unwrap();

            assert_eq!(cx, rx);
        }
    }

    #[test]
    fn ft232h_test_loopback_t2() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let mut spidev = dev.spi().unwrap();

        // loopback: 3-byte messages
        for (x, y, z) in iproduct!(1..5, 11..15, 21..25) {
            let mut tx = [x, y, z];
            let cx = tx;
            let rx = spidev.transfer(&mut tx).unwrap();
            assert_eq!(cx, rx);
        }
    }

    #[test]
    fn ft232h_test_loopback_t3() {
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
    fn ft232h_test_loopback_multi_bus_t1() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let mut spidev1 = dev.spi().unwrap();
        let mut spidev2 = dev.spi().unwrap();

        // loopback: 1-byte messages on both protocol buses
        for v in 0x0..0xff {
            let mut tx1 = [v; 1];
            let cx1 = tx1;

            let mut tx2 = [v; 1];
            let cx2 = tx2;

            let rx1 = spidev1.transfer(&mut tx1).unwrap();
            let rx2 = spidev2.transfer(&mut tx2).unwrap();

            assert_eq!(cx1, rx1);
            assert_eq!(cx2, rx2);
        }
    }
}
