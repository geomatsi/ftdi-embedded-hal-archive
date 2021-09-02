fn main() {
    println!("Run example tests: cargo test --example=at24c04-test1 -- --test-threads=1");
}

#[cfg(test)]
mod test {
    use eeprom24x::Eeprom24x;
    use eeprom24x::SlaveAddr;
    use ftdi_embedded_hal as hal;
    use ftdi_embedded_hal::x232h::FTx232H;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn at24x_test_t1() {
        #[cfg(all(feature = "ftdi-lib", feature = "ftd2-lib"))]
        compile_error!("features 'ftdi-lib' and 'ftd2-lib' cannot be enabled at the same time");

        #[cfg(not(any(feature = "ftdi-lib", feature = "ftd2-lib")))]
        compile_error!("one of features 'ftdi-lib' and 'ftd2-lib' shall be enabled");

        #[cfg(feature = "ftdi-lib")]
        let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
            .interface(ftdi::Interface::A)
            .open()
            .unwrap();

        #[cfg(feature = "ftd2-lib")]
        let device = libftd2xx::Ft232h::with_description("Single RS232-HS").unwrap();

        let hal = hal::x232h::FTx232H::init(device, 400_000).unwrap();
        let i2c = hal.i2c().unwrap();
        let mut eeprom = Eeprom24x::new_24x04(i2c, SlaveAddr::default());

        let delay = Duration::from_millis(5);
        let byte_w = 0xe5;
        let addr = 0x0;

        eeprom.write_byte(addr, byte_w).unwrap();
        sleep(delay);

        let byte_r = eeprom.read_byte(addr).unwrap();

        assert_eq!(byte_w, byte_r);
    }

    #[test]
    fn at24x_test_t2() {
        #[cfg(all(feature = "ftdi-lib", feature = "ftd2-lib"))]
        compile_error!("features 'ftdi-lib' and 'ftd2-lib' cannot be enabled at the same time");

        #[cfg(not(any(feature = "ftdi-lib", feature = "ftd2-lib")))]
        compile_error!("one of features 'ftdi-lib' and 'ftd2-lib' shall be enabled");

        #[cfg(feature = "ftdi-lib")]
        let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
            .interface(ftdi::Interface::A)
            .open()
            .unwrap();

        #[cfg(feature = "ftd2-lib")]
        let device = libftd2xx::Ft232h::with_description("Single RS232-HS").unwrap();

        let hal = hal::x232h::FTx232H::init(device, 400_000).unwrap();
        let i2c = hal.i2c().unwrap();
        let mut eeprom = Eeprom24x::new_24x04(i2c, SlaveAddr::default());

        let delay = Duration::from_millis(5);
        let data_w: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
        let mut data_r: [u8; 4] = [0; 4];

        for i in 0..data_w.len() {
            eeprom.write_byte(i as u32, data_w[i]).unwrap();
            sleep(delay);
        }

        for i in 0..data_r.len() {
            data_r[i] = eeprom.read_byte(i as u32).unwrap();
        }

        assert_eq!(data_w, data_r);
    }

    #[test]
    fn at24x_test_t3() {
        #[cfg(all(feature = "ftdi-lib", feature = "ftd2-lib"))]
        compile_error!("features 'ftdi-lib' and 'ftd2-lib' cannot be enabled at the same time");

        #[cfg(not(any(feature = "ftdi-lib", feature = "ftd2-lib")))]
        compile_error!("one of features 'ftdi-lib' and 'ftd2-lib' shall be enabled");

        #[cfg(feature = "ftdi-lib")]
        let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
            .interface(ftdi::Interface::A)
            .open()
            .unwrap();

        #[cfg(feature = "ftd2-lib")]
        let device = libftd2xx::Ft232h::with_description("Single RS232-HS").unwrap();

        let hal = hal::x232h::FTx232H::init(device, 400_000).unwrap();
        let i2c = hal.i2c().unwrap();
        let mut eeprom = Eeprom24x::new_24x04(i2c, SlaveAddr::default());

        let delay = Duration::from_millis(5);
        let data_w: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
        let mut data_r: [u8; 4] = [0; 4];

        for i in 0..data_w.len() {
            eeprom.write_byte(i as u32, data_w[i]).unwrap();
            sleep(delay);
        }

        eeprom.read_data(0x0, &mut data_r).unwrap();

        assert_eq!(data_w, data_r);
    }

    #[test]
    fn at24x_test_t4() {
        #[cfg(all(feature = "ftdi-lib", feature = "ftd2-lib"))]
        compile_error!("features 'ftdi-lib' and 'ftd2-lib' cannot be enabled at the same time");

        #[cfg(not(any(feature = "ftdi-lib", feature = "ftd2-lib")))]
        compile_error!("one of features 'ftdi-lib' and 'ftd2-lib' shall be enabled");

        #[cfg(feature = "ftdi-lib")]
        let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
            .interface(ftdi::Interface::A)
            .open()
            .unwrap();

        #[cfg(feature = "ftd2-lib")]
        let device = libftd2xx::Ft232h::with_description("Single RS232-HS").unwrap();

        let hal = hal::x232h::FTx232H::init(device, 400_000).unwrap();
        let i2c = hal.i2c().unwrap();
        let mut eeprom = Eeprom24x::new_24x04(i2c, SlaveAddr::default());

        let delay = Duration::from_millis(50);
        let addrs: [u32; 4] = [0x00, 0x10, 0x20, 0x30];
        let mut data_r = [0x00; 16];
        let data_w = [0xAB; 16];

        for addr in addrs.iter() {
            eeprom.write_page(*addr, &data_w).unwrap();
            sleep(delay);
            eeprom.read_data(*addr, &mut data_r).unwrap();
            assert_eq!(data_w, data_r);
        }
    }

    #[test]
    fn at24x_test_t5() {
        #[cfg(all(feature = "ftdi-lib", feature = "ftd2-lib"))]
        compile_error!("features 'ftdi-lib' and 'ftd2-lib' cannot be enabled at the same time");

        #[cfg(not(any(feature = "ftdi-lib", feature = "ftd2-lib")))]
        compile_error!("one of features 'ftdi-lib' and 'ftd2-lib' shall be enabled");

        #[cfg(feature = "ftdi-lib")]
        let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
            .interface(ftdi::Interface::A)
            .open()
            .unwrap();

        #[cfg(feature = "ftd2-lib")]
        let device = libftd2xx::Ft232h::with_description("Single RS232-HS").unwrap();

        let hal = hal::x232h::FTx232H::init(device, 400_000).unwrap();
        let i2c = hal.i2c().unwrap();
        let mut eeprom = Eeprom24x::new_24x04(i2c, SlaveAddr::default());
        let delay = Duration::from_millis(5);

        // check high memory addresses: 1 bit passed as a part of i2c addr
        let addrs1: [u32; 4] = [0x100, 0x10F, 0x1F0, 0x1EE];
        let byte_w1 = 0xe5;
        let addrs2: [u32; 4] = [0x00, 0x0F, 0xF0, 0xEE];
        let byte_w2 = 0xaa;

        // write bytes

        for addr in addrs1.iter() {
            eeprom.write_byte(*addr, byte_w1).unwrap();
            sleep(delay);
        }

        for addr in addrs2.iter() {
            eeprom.write_byte(*addr, byte_w2).unwrap();
            sleep(delay);
        }

        // read bytes and check

        for addr in addrs1.iter() {
            let byte_r = eeprom.read_byte(*addr).unwrap();
            assert_eq!(byte_w1, byte_r);
            sleep(delay);
        }

        for addr in addrs2.iter() {
            let byte_r = eeprom.read_byte(*addr).unwrap();
            assert_eq!(byte_w2, byte_r);
            sleep(delay);
        }
    }
}
