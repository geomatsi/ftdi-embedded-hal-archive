extern crate eeprom24x;
extern crate ftdi_embedded_hal as hal;

fn main() {
    println!("Run example tests: cargo test --example=at24c04-test1 -- --test-threads=1");
}

#[cfg(test)]
mod test {
    use crate::hal::ft232h::FT232H;

    use eeprom24x::Eeprom24x;
    use eeprom24x::SlaveAddr;

    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn at24x_test_t1() {
        let dev = FT232H::init(0x0403, 0x6014).unwrap();
        let i2c = dev.i2c().unwrap();
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
        let dev = FT232H::init(0x0403, 0x6014).unwrap();
        let i2c = dev.i2c().unwrap();
        let mut eeprom = Eeprom24x::new_24x04(i2c, SlaveAddr::default());

        let delay = Duration::from_millis(5);
        let data_w: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
        let mut data_r: [u8; 4] = [0; 4];

        for i in 0..data_w.len() {
            eeprom.write_byte(i as u8, data_w[i]).unwrap();
            sleep(delay);
        }

        for i in 0..data_r.len() {
            data_r[i] = eeprom.read_byte(i as u8).unwrap();
        }

        assert_eq!(data_w, data_r);
    }

    #[test]
    fn at24x_test_t3() {
        let dev = FT232H::init(0x0403, 0x6014).unwrap();
        let i2c = dev.i2c().unwrap();
        let mut eeprom = Eeprom24x::new_24x04(i2c, SlaveAddr::default());

        let delay = Duration::from_millis(5);
        let data_w: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
        let mut data_r: [u8; 4] = [0; 4];

        for i in 0..data_w.len() {
            eeprom.write_byte(i as u8, data_w[i]).unwrap();
            sleep(delay);
        }

        eeprom.read_data(0x0, &mut data_r).unwrap();

        assert_eq!(data_w, data_r);
    }
}
