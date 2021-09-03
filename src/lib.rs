pub mod error;
pub mod gpio;
pub mod i2c;
pub mod spi;
pub mod x232h;

macro_rules! ftdi_test_suite {
    ($device: expr) => {
        use crate::gpio::PinBank;
        use crate::x232h::FTx232H;
        use embedded_hal::blocking::spi::Transfer;
        use embedded_hal::spi::{MODE_0, MODE_1, MODE_2, MODE_3};
        use itertools::iproduct;
        use rand::Rng;

        #[test]
        fn test_init_t1() {
            let device = $device;
            let mut hal = FTx232H::init(device, 3_000_000).unwrap();

            assert_eq!(hal.is_loopback(), false);
            hal.loopback(true).unwrap();
            assert_eq!(hal.is_loopback(), true);
        }

        #[test]
        fn test_init_t2() {
            let device = $device;
            let hal = FTx232H::init(device, 3_000_000).unwrap();

            let pl0 = hal.pl0().unwrap();
            assert_eq!(pl0.get_bit(), 4);
            assert_eq!(pl0.get_bank(), PinBank::Low);
            assert_eq!(format!("{}", pl0), "PL0");

            let pl1 = hal.pl1().unwrap();
            assert_eq!(pl1.get_bit(), 5);
            assert_eq!(pl1.get_bank(), PinBank::Low);
            assert_eq!(format!("{}", pl1), "PL1");

            let pl2 = hal.pl2().unwrap();
            assert_eq!(pl2.get_bit(), 6);
            assert_eq!(pl2.get_bank(), PinBank::Low);
            assert_eq!(format!("{}", pl2), "PL2");

            let pl3 = hal.pl3().unwrap();
            assert_eq!(pl3.get_bit(), 7);
            assert_eq!(pl3.get_bank(), PinBank::Low);
            assert_eq!(format!("{}", pl3), "PL3");
        }

        #[test]
        fn test_init_t3() {
            let device = $device;
            let hal = FTx232H::init(device, 3_000_000).unwrap();

            let ph0 = hal.ph0().unwrap();
            assert_eq!(ph0.get_bit(), 0);
            assert_eq!(ph0.get_bank(), PinBank::High);
            assert_eq!(format!("{}", ph0), "PH0");

            let ph1 = hal.ph1().unwrap();
            assert_eq!(ph1.get_bit(), 1);
            assert_eq!(ph1.get_bank(), PinBank::High);
            assert_eq!(format!("{}", ph1), "PH1");

            let ph2 = hal.ph2().unwrap();
            assert_eq!(ph2.get_bit(), 2);
            assert_eq!(ph2.get_bank(), PinBank::High);
            assert_eq!(format!("{}", ph2), "PH2");

            let ph3 = hal.ph3().unwrap();
            assert_eq!(ph3.get_bit(), 3);
            assert_eq!(ph3.get_bank(), PinBank::High);
            assert_eq!(format!("{}", ph3), "PH3");

            let ph4 = hal.ph4().unwrap();
            assert_eq!(ph4.get_bit(), 4);
            assert_eq!(ph4.get_bank(), PinBank::High);
            assert_eq!(format!("{}", ph4), "PH4");

            let ph5 = hal.ph5().unwrap();
            assert_eq!(ph5.get_bit(), 5);
            assert_eq!(ph5.get_bank(), PinBank::High);
            assert_eq!(format!("{}", ph5), "PH5");

            let ph6 = hal.ph6().unwrap();
            assert_eq!(ph6.get_bit(), 6);
            assert_eq!(ph6.get_bank(), PinBank::High);
            assert_eq!(format!("{}", ph6), "PH6");

            let ph7 = hal.ph7().unwrap();
            assert_eq!(ph7.get_bit(), 7);
            assert_eq!(ph7.get_bank(), PinBank::High);
            assert_eq!(format!("{}", ph7), "PH7");
        }

        #[test]
        fn test_init_t4() {
            let device = $device;
            let hal = FTx232H::init(device, 3_000_000).unwrap();

            assert_eq!(hal.is_loopback(), false);

            let ph0_0 = hal.ph0();
            let ph0_1 = hal.ph0();
            let ph0_2 = hal.ph0();

            assert!(ph0_0.is_ok(), "First pin instance should be OK");
            assert!(ph0_1.is_err(), "There should be no second pin instance");
            assert!(ph0_2.is_err(), "There should be no third pin instance");
        }

        #[test]
        fn test_init_t5() {
            let device = $device;
            let hal = FTx232H::init(device, 3_000_000).unwrap();
            let mut spidev = hal.spi().unwrap();

            assert_eq!(hal.is_loopback(), false);

            let res = spidev.set_mode(MODE_0);
            assert!(res.is_ok(), "Can't set SPI MODE0");

            let res = spidev.set_mode(MODE_1);
            assert!(res.is_err(), "SPI MODE1 should not be supported");

            let res = spidev.set_mode(MODE_2);
            assert!(res.is_ok(), "Can't set SPI MODE2");

            let res = spidev.set_mode(MODE_3);
            assert!(res.is_err(), "SPI MODE3 should not be supported");
        }

        #[test]
        fn test_init_t6() {
            let device = $device;
            let hal = FTx232H::init(device, 3_000_000).unwrap();

            assert_eq!(hal.is_loopback(), false);

            let spi1 = hal.spi();
            assert!(spi1.is_ok(), "1st spi instance should be ok");

            let i2c = hal.i2c();
            assert!(i2c.is_err(), "i2c instance after spi should not be ok");

            let spi2 = hal.spi();
            assert!(spi2.is_ok(), "2nd spi instance should be ok");
        }

        #[test]
        fn test_init_t7() {
            let device = $device;
            let hal = FTx232H::init(device, 3_000_000).unwrap();

            assert_eq!(hal.is_loopback(), false);

            let i2c1 = hal.i2c();
            assert!(i2c1.is_ok(), "1st i2c instance should be ok");

            let spi = hal.spi();
            assert!(spi.is_err(), "spi instance after i2c should not be ok");

            let i2c2 = hal.i2c();
            assert!(i2c2.is_ok(), "2nd i2c instance should be ok");
        }

        #[test]
        fn test_init_t8() {
            let device = $device;
            let hal = FTx232H::init(device, 3_000_000).unwrap();

            assert_eq!(hal.is_loopback(), false);

            let spi1 = hal.spi();
            assert!(spi1.is_ok(), "1st spi instance should be ok");

            let spi2 = hal.spi();
            assert!(spi2.is_ok(), "2nd spi instance should be ok");

            let spi3 = hal.spi();
            assert!(spi3.is_ok(), "3rd spi should be ok");

            let spi4 = hal.spi();
            assert!(spi4.is_ok(), "3rd spi should be ok");
        }

        #[test]
        fn test_init_t9() {
            let device = $device;
            let hal = FTx232H::init(device, 3_000_000).unwrap();

            assert_eq!(hal.is_loopback(), false);

            let i2c1 = hal.i2c();
            assert!(i2c1.is_ok(), "1st i2c instance should be ok");

            let i2c2 = hal.i2c();
            assert!(i2c2.is_ok(), "2nd i2c instance should be ok");

            let i2c3 = hal.i2c();
            assert!(i2c3.is_ok(), "3rd i2c instance should be ok");

            let i2c4 = hal.i2c();
            assert!(i2c4.is_ok(), "4th i2c instance should be ok");
        }

        #[test]
        fn test_loopback_t1() {
            let device = $device;
            let mut hal = FTx232H::init(device, 3_000_000).unwrap();

            hal.loopback(true).unwrap();
            assert_eq!(hal.is_loopback(), true);

            let mut spidev = hal.spi().unwrap();

            // loopback: 1-byte messages
            for v in 0x0..0xff {
                let mut tx = [v; 1];
                let cx = tx;
                let rx = spidev.transfer(&mut tx).unwrap();

                assert_eq!(cx, rx);
            }
        }

        #[test]
        fn test_loopback_t2() {
            let device = $device;
            let mut hal = FTx232H::init(device, 3_000_000).unwrap();

            hal.loopback(true).unwrap();
            assert_eq!(hal.is_loopback(), true);

            let mut spidev = hal.spi().unwrap();

            // loopback: 3-byte messages
            for (x, y, z) in iproduct!(1..5, 11..15, 21..25) {
                let mut tx = [x, y, z];
                let cx = tx;
                let rx = spidev.transfer(&mut tx).unwrap();
                assert_eq!(cx, rx);
            }
        }

        #[test]
        fn test_loopback_t3() {
            let device = $device;
            let mut hal = FTx232H::init(device, 3_000_000).unwrap();

            hal.loopback(true).unwrap();
            assert_eq!(hal.is_loopback(), true);

            let mut spidev = hal.spi().unwrap();

            // loopback: 5-byte random messages
            for _ in 1..10 {
                let mut rng = rand::thread_rng();
                let mut tx: Vec<u8> = (0..5)
                    .map(|_| {
                        // 0 (inclusive) to 254 (inclusive)
                        rng.gen_range(0..255)
                    })
                    .collect();
                let cx = tx.clone();
                let rx = spidev.transfer(&mut tx).unwrap();
                assert_eq!(cx, rx);
            }
        }

        #[test]
        fn test_loopback_multi_bus_t1() {
            let device = $device;
            let mut hal = FTx232H::init(device, 3_000_000).unwrap();

            hal.loopback(true).unwrap();
            assert_eq!(hal.is_loopback(), true);

            let mut spidev1 = hal.spi().unwrap();
            let mut spidev2 = hal.spi().unwrap();

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
    };
}

#[cfg(test)]
mod tests {
    mod ftdi {
        mod ft232h {
            ftdi_test_suite!(ftdi::find_by_vid_pid(0x0403, 0x6014)
                .interface(ftdi::Interface::A)
                .open()
                .unwrap());
        }

        mod ft2232h_a {
            ftdi_test_suite!(ftdi::find_by_vid_pid(0x0403, 0x6010)
                .interface(ftdi::Interface::A)
                .open()
                .unwrap());
        }

        mod ft2232h_b {
            ftdi_test_suite!(ftdi::find_by_vid_pid(0x0403, 0x6010)
                .interface(ftdi::Interface::B)
                .open()
                .unwrap());
        }
    }

    mod libftd2xx {
        mod ft232h {
            ftdi_test_suite!(libftd2xx::Ft232h::with_description("Single RS232-HS").unwrap());
        }

        mod ft2232h_a {
            ftdi_test_suite!(libftd2xx::Ft2232h::with_description("Dual RS232-HS A").unwrap());
        }

        mod ft2232h_b {
            ftdi_test_suite!(libftd2xx::Ft2232h::with_description("Dual RS232-HS B").unwrap());
        }
    }
}
