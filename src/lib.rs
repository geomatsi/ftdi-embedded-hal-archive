pub mod error;
pub mod gpio;
pub mod i2c;
pub mod spi;
pub mod x232h;

#[cfg(test)]
mod tests {
    macro_rules! ftdi_test_suite {
        ($vendor: expr, $product: expr, $channel: expr) => {
            use crate::gpio::PinBank;
            use crate::i2c::I2cSpeed;
            use crate::spi::SpiSpeed;
            use crate::x232h::FTx232H;
            use crate::x232h::Interface;
            use embedded_hal::blocking::spi::Transfer;
            use embedded_hal::spi::{MODE_0, MODE_1, MODE_2, MODE_3};
            use itertools::iproduct;
            use rand::Rng;

            #[test]
            fn test_init_t1() {
                let mut dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                dev.loopback(true).unwrap();
                assert_eq!(dev.is_loopback(), true);
            }

            #[test]
            fn test_init_t2() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();

                let pl0 = dev.pl0().unwrap();
                assert_eq!(pl0.get_bit(), 4);
                assert_eq!(pl0.get_bank(), PinBank::Low);
                assert_eq!(format!("{}", pl0), "PL0");

                let pl1 = dev.pl1().unwrap();
                assert_eq!(pl1.get_bit(), 5);
                assert_eq!(pl1.get_bank(), PinBank::Low);
                assert_eq!(format!("{}", pl1), "PL1");

                let pl2 = dev.pl2().unwrap();
                assert_eq!(pl2.get_bit(), 6);
                assert_eq!(pl2.get_bank(), PinBank::Low);
                assert_eq!(format!("{}", pl2), "PL2");

                let pl3 = dev.pl3().unwrap();
                assert_eq!(pl3.get_bit(), 7);
                assert_eq!(pl3.get_bank(), PinBank::Low);
                assert_eq!(format!("{}", pl3), "PL3");
            }

            #[test]
            fn test_init_t3() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();

                let ph0 = dev.ph0().unwrap();
                assert_eq!(ph0.get_bit(), 0);
                assert_eq!(ph0.get_bank(), PinBank::High);
                assert_eq!(format!("{}", ph0), "PH0");

                let ph1 = dev.ph1().unwrap();
                assert_eq!(ph1.get_bit(), 1);
                assert_eq!(ph1.get_bank(), PinBank::High);
                assert_eq!(format!("{}", ph1), "PH1");

                let ph2 = dev.ph2().unwrap();
                assert_eq!(ph2.get_bit(), 2);
                assert_eq!(ph2.get_bank(), PinBank::High);
                assert_eq!(format!("{}", ph2), "PH2");

                let ph3 = dev.ph3().unwrap();
                assert_eq!(ph3.get_bit(), 3);
                assert_eq!(ph3.get_bank(), PinBank::High);
                assert_eq!(format!("{}", ph3), "PH3");

                let ph4 = dev.ph4().unwrap();
                assert_eq!(ph4.get_bit(), 4);
                assert_eq!(ph4.get_bank(), PinBank::High);
                assert_eq!(format!("{}", ph4), "PH4");

                let ph5 = dev.ph5().unwrap();
                assert_eq!(ph5.get_bit(), 5);
                assert_eq!(ph5.get_bank(), PinBank::High);
                assert_eq!(format!("{}", ph5), "PH5");

                let ph6 = dev.ph6().unwrap();
                assert_eq!(ph6.get_bit(), 6);
                assert_eq!(ph6.get_bank(), PinBank::High);
                assert_eq!(format!("{}", ph6), "PH6");

                let ph7 = dev.ph7().unwrap();
                assert_eq!(ph7.get_bit(), 7);
                assert_eq!(ph7.get_bank(), PinBank::High);
                assert_eq!(format!("{}", ph7), "PH7");
            }

            #[test]
            fn test_init_t4() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                let ph0_0 = dev.ph0();
                let ph0_1 = dev.ph0();
                let ph0_2 = dev.ph0();

                assert!(ph0_0.is_ok(), "First pin instance should be OK");
                assert!(ph0_1.is_err(), "There should be no second pin instance");
                assert!(ph0_2.is_err(), "There should be no third pin instance");
            }

            #[test]
            fn test_init_t5() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                let mut spidev = dev.spi(SpiSpeed::CLK_AUTO).unwrap();

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
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                let spi1 = dev.spi(SpiSpeed::CLK_AUTO);
                assert!(spi1.is_ok(), "1st spi instance should be ok");

                let i2c = dev.i2c(I2cSpeed::CLK_AUTO);
                assert!(i2c.is_err(), "i2c instance after spi should not be ok");

                let spi2 = dev.spi(SpiSpeed::CLK_AUTO);
                assert!(spi2.is_ok(), "2nd spi instance should be ok");
            }

            #[test]
            fn test_init_t7() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                let i2c1 = dev.i2c(I2cSpeed::CLK_AUTO);
                assert!(i2c1.is_ok(), "1st i2c instance should be ok");

                let spi = dev.spi(SpiSpeed::CLK_AUTO);
                assert!(spi.is_err(), "spi instance after i2c should not be ok");

                let i2c2 = dev.i2c(I2cSpeed::CLK_AUTO);
                assert!(i2c2.is_ok(), "2nd i2c instance should be ok");
            }

            #[test]
            fn test_init_t8() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                let spi1 = dev.spi(SpiSpeed::CLK_1MHz);
                assert!(spi1.is_ok(), "1st spi instance should be ok");

                let spi2 = dev.spi(SpiSpeed::CLK_1MHz);
                assert!(
                    spi2.is_ok(),
                    "2nd spi instance with the same clock should be ok"
                );

                let spi3 = dev.spi(SpiSpeed::CLK_3MHz);
                assert!(
                    spi3.is_err(),
                    "3rd spi failure: clock should be the same or auto"
                );

                let spi4 = dev.spi(SpiSpeed::CLK_AUTO);
                assert!(spi4.is_ok(), "4th spi with AUTO clock should be ok");
            }

            #[test]
            fn test_init_t9() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                let spi1 = dev.spi(SpiSpeed::CLK_AUTO);
                assert!(spi1.is_ok(), "1st spi instance should be ok");

                let spi2 = dev.spi(SpiSpeed::CLK_1MHz);
                assert!(
                    spi2.is_err(),
                    "2nd spi instance with non-AUTO clock should fail"
                );

                let spi3 = dev.spi(SpiSpeed::CLK_AUTO);
                assert!(spi3.is_ok(), "3rd spi with AUTO clock should be ok");
            }

            #[test]
            fn test_init_t10() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                let i2c1 = dev.i2c(I2cSpeed::CLK_100kHz);
                assert!(i2c1.is_ok(), "1st i2c instance should be ok");

                let i2c2 = dev.i2c(I2cSpeed::CLK_100kHz);
                assert!(
                    i2c2.is_ok(),
                    "2nd i2c instance with the same clock should be ok"
                );

                let i2c3 = dev.i2c(I2cSpeed::CLK_400kHz);
                assert!(
                    i2c3.is_err(),
                    "3rd i2c failure: clk should be the same or auto"
                );

                let i2c4 = dev.i2c(I2cSpeed::CLK_AUTO);
                assert!(i2c4.is_ok(), "4th i2c with AUTO clock should be ok");
            }

            #[test]
            fn test_init_t11() {
                let dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                assert_eq!(dev.is_loopback(), false);

                let i2c1 = dev.i2c(I2cSpeed::CLK_AUTO);
                assert!(i2c1.is_ok(), "1st i2c instance should be ok");

                let i2c2 = dev.i2c(I2cSpeed::CLK_400kHz);
                assert!(
                    i2c2.is_err(),
                    "2nd i2c instance with non-AUTO clock should fail"
                );

                let i2c3 = dev.i2c(I2cSpeed::CLK_AUTO);
                assert!(i2c3.is_ok(), "3rd i2c with AUTO clock should be ok");
            }

            #[test]
            fn test_loopback_t1() {
                let mut dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                dev.loopback(true).unwrap();
                assert_eq!(dev.is_loopback(), true);

                let mut spidev = dev.spi(SpiSpeed::CLK_AUTO).unwrap();

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
                let mut dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                dev.loopback(true).unwrap();
                assert_eq!(dev.is_loopback(), true);

                let mut spidev = dev.spi(SpiSpeed::CLK_AUTO).unwrap();

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
                let mut dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                dev.loopback(true).unwrap();
                assert_eq!(dev.is_loopback(), true);

                let mut spidev = dev.spi(SpiSpeed::CLK_AUTO).unwrap();

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
                let mut dev = FTx232H::init_chan($vendor, $product, $channel).unwrap();
                dev.loopback(true).unwrap();
                assert_eq!(dev.is_loopback(), true);

                let mut spidev1 = dev.spi(SpiSpeed::CLK_AUTO).unwrap();
                let mut spidev2 = dev.spi(SpiSpeed::CLK_AUTO).unwrap();

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

    mod ft232h {
        ftdi_test_suite!(0x0403, 0x6014, Interface::A);
    }

    mod ft2232h_a {
        ftdi_test_suite!(0x0403, 0x6010, Interface::A);
    }

    mod ft2232h_b {
        ftdi_test_suite!(0x0403, 0x6010, Interface::B);
    }
}
