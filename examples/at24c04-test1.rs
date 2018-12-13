extern crate ftdi_embedded_hal as hal;
use crate::hal::ft232h::FT232H;

extern crate eeprom24x;
use eeprom24x::Eeprom24x;
use eeprom24x::SlaveAddr;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let dev = FT232H::init(0x0403, 0x6014).unwrap();
    let i2c = dev.i2c().unwrap();
    let mut eeprom = Eeprom24x::new_24x04(i2c, SlaveAddr::default());

    let five_ms = Duration::from_millis(100);
    let addr = [0x1, 0x0];
    let d_out = 0xe5;
    eeprom.write_byte(addr, d_out).unwrap();
    sleep(five_ms);
    let d_in = eeprom.read_byte(addr).unwrap();
    println!("out 0x{:x} in 0x{:x}", d_out, d_in);

    //let delay = Duration::from_millis(100);
    //let data = eeprom.read_byte([0x0, 0x0]).unwrap();
    //println!("data 0x{:x}", data);
    //sleep(delay);
    //let data = eeprom.read_byte([0x1, 0x0]).unwrap();
    //println!("data 0x{:x}", data);
    //sleep(delay);
    //let data = eeprom.read_byte([0x2, 0x0]).unwrap();
    //println!("data 0x{:x}", data);
    //sleep(delay);
    //let data = eeprom.read_byte([0x3, 0x0]).unwrap();
    //println!("data 0x{:x}", data);
    //sleep(delay);

    //let addr = [0x0, 0x0];
    //let mut data = [0; 4];
    //eeprom.read_data(addr, &mut data).unwrap();
    //println!("data 0x{:x} 0x{:x} 0x{:x} 0x{:x}", data[0], data[1], data[2], data[3]);

    //let delay = Duration::from_millis(100);
    //eeprom.write_byte([0x0, 0x0], 0xaa).unwrap();
    //sleep(delay);
    //eeprom.write_byte([0x1, 0x0], 0xbb).unwrap();
    //sleep(delay);
    //eeprom.write_byte([0x2, 0x0], 0xcc).unwrap();
    //sleep(delay);
    //eeprom.write_byte([0x3, 0x0], 0xdd).unwrap();
    //sleep(delay);

}
