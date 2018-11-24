#![feature(extern_crate_item_prelude)]

extern crate ftdi_embedded_hal as hal;

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::OutputPin;

use hal::devices::{FtdiDevice, FtdiPin};

fn main() {
    let regs: Vec<u8> = vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];
    let mut dev = FtdiDevice::spi_init(0x0403, 0x6014, None).unwrap();

    dev.select_pin(FtdiPin::PinH0);

    // This example refers to specific schematics:
    // nRF24 CSN pin is connected to PinL2 rather than TMS/CS pin
    for r in regs {
        dev.select_pin(FtdiPin::PinL2).set_low();

        // send command: read register r
        let mut cmd = [0x00 | (0x1F & r); 1];
        dev.transfer(&mut cmd).unwrap();

        // send dummy value: read previous cmd result
        let mut dummy = [0xff];
        let regval = dev.transfer(&mut dummy).unwrap();

        dev.select_pin(FtdiPin::PinL2).set_high();

        println!("REG[0x{:x}] = [{:08b}]", r, regval[0]);
    }
}
