#![feature(extern_crate_item_prelude)]

use std::io::{Result, Error};

extern crate ftdi_embedded_hal as ftdi_hal;
use ftdi_hal::devices::{FtdiDevice, FtdiPin};

extern crate embedded_hal as hal;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::OutputPin;

extern crate embedded_nrf24l01;
use embedded_nrf24l01::Configuration;
use embedded_nrf24l01::CrcMode;
use embedded_nrf24l01::DataRate;
use embedded_nrf24l01::NRF24L01;

struct FtdiProxy;

impl FtdiProxy {
    pub fn new(ftdi: &mut FtdiDevice) -> FtdiProxy {
       FtdiProxy {} 
    }
}

impl hal::blocking::spi::Transfer<u8> for FtdiProxy {
    type Error = Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8]> {
        Ok(&[])
    }
}

impl hal::digital::OutputPin for FtdiProxy {
    fn set_low(&mut self) {

    }

    fn set_high(&mut self) {

    }
}


fn main() {
    let regs: Vec<u8> = vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];
    let mut dev = FtdiDevice::spi_init(0x0403, 0x6014, None).unwrap();

    let ce = FtdiProxy::new(&mut dev);
    let cs = FtdiProxy::new(&mut dev);
    let spi = FtdiProxy::new(&mut dev);

    // nRF24L01 setup
    let mut nrf = NRF24L01::new(ce, cs, spi).unwrap();
    nrf.set_frequency(120).unwrap();
    nrf.set_rf(DataRate::R250Kbps, 3 /* 0 dBm */).unwrap();
    nrf.set_crc(Some(CrcMode::OneByte)).unwrap();
    nrf.set_auto_retransmit(0b0100, 0b1111).unwrap();

    let addr: [u8; 5] = [0xe5, 0xe4, 0xe3, 0xe2, 0xe1];
    nrf.set_rx_addr(0, &addr).unwrap();
    nrf.set_tx_addr(&addr).unwrap();
    nrf.set_pipes_rx_lengths(&[None; 6]).unwrap();
    nrf.flush_tx().unwrap();
    nrf.flush_rx().unwrap();



    //
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
