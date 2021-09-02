use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin};
use ftdi_embedded_hal as hal;

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

    let hal = hal::x232h::FTx232H::init(device, 1_000_000).unwrap();
    let mut spidev = hal.spi().unwrap();
    let mut pl2 = hal.pl2().unwrap();

    let regs: Vec<u8> = vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];

    // This example refers to specific schematics:
    // nRF24 CSN pin is connected to PinL2 rather than TMS/CS pin
    for r in regs {
        pl2.set_low().unwrap();

        // send command: read register r
        let mut cmd = [0x1F & r; 1];
        spidev.transfer(&mut cmd).unwrap();

        // send dummy value: read previous cmd result
        let mut dummy = [0xff];
        let regval = spidev.transfer(&mut dummy).unwrap();

        pl2.set_high().unwrap();

        println!("REG[0x{:x}] = [{:08b}]", r, regval[0]);
    }
}
