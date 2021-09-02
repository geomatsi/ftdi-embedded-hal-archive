use ftdi_embedded_hal as hal;
use lm75::{Lm75, SlaveAddr};
use std::thread::sleep;
use std::time::Duration;

fn main() {
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
    let mut sensor = Lm75::new(i2c, SlaveAddr::default());
    let delay = Duration::from_secs(1);

    for _ in 0..5 {
        let temperature = sensor.read_temperature().unwrap();
        println!("Temperature: {}", temperature);
        sleep(delay);
    }
}
