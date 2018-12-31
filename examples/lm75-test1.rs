extern crate ftdi_embedded_hal as hal;
extern crate lm75;

use crate::hal::x232h::FTx232H;

use lm75::{Lm75, SlaveAddr};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let dev = FTx232H::init(0x0403, 0x6014).unwrap();
    let i2c = dev.i2c().unwrap();
    let mut sensor = Lm75::new(i2c, SlaveAddr::default());
    let delay = Duration::from_secs(1);

    for _ in 0..5 {
        let temperature = sensor.read_temperature().unwrap();
        println!("Temperature: {}", temperature);
        sleep(delay);
    }
}
