#![feature(extern_crate_item_prelude)]

extern crate ftdi_embedded_hal as hal;

use embedded_hal::blocking::spi::Transfer;
use hal::devices::FtdiDevice;

fn main() {
    let mut dev = FtdiDevice::spi_init(0x0403, 0x6014, None).unwrap();
    let regs: Vec<u8> = vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];

    hal::gpio::um232_spi_csn(&mut dev.ctx, 0x20, 0x0).unwrap();

    for r in regs {
        hal::gpio::um232_spi_csn(&mut dev.ctx, 0x40, 0x0).unwrap();

        // send command: read register r
        let mut cmd = [0x00 | (0x1F & r); 1];
        dev.transfer(&mut cmd).unwrap();
        // send dummy value: read previous cmd result
        let mut dummy = [0xff];
        let regval = dev.transfer(&mut dummy).unwrap();

        hal::gpio::um232_spi_csn(&mut dev.ctx, 0x40, 0x1).unwrap();
        println!("REG[0x{:x}] = [{:08b}]", r, regval[0]);
    }
}
