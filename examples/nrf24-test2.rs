#![feature(extern_crate_item_prelude)]

use std::cell::RefCell;
use std::io::{Error, Result};
use std::rc::Rc;

extern crate ftdi_embedded_hal as ftdi_hal;
use ftdi_hal::devices::{FtdiDevice, FtdiPin};

extern crate embedded_hal as hal;

extern crate embedded_nrf24l01;
use embedded_nrf24l01::Configuration;
use embedded_nrf24l01::CrcMode;
use embedded_nrf24l01::DataRate;
use embedded_nrf24l01::NRF24L01;

//

struct CS {
    ftdi: Rc<RefCell<FtdiDevice>>,
}

impl CS {
    pub fn new(ftdi: Rc<RefCell<FtdiDevice>>) -> CS {
        CS { ftdi: ftdi }
    }
}

impl hal::digital::OutputPin for CS {
    fn set_low(&mut self) {
        self.ftdi.borrow_mut().select_pin(FtdiPin::PinL2).set_low();
    }

    fn set_high(&mut self) {
        self.ftdi.borrow_mut().select_pin(FtdiPin::PinL2).set_high();
    }
}

//

struct CE {
    ftdi: Rc<RefCell<FtdiDevice>>,
}

impl CE {
    pub fn new(ftdi: Rc<RefCell<FtdiDevice>>) -> CE {
        CE { ftdi: ftdi }
    }
}

impl hal::digital::OutputPin for CE {
    fn set_low(&mut self) {
        self.ftdi.borrow_mut().select_pin(FtdiPin::PinH0).set_low();
    }

    fn set_high(&mut self) {
        self.ftdi.borrow_mut().select_pin(FtdiPin::PinH0).set_high();
    }
}

//

struct SPI {
    ftdi: Rc<RefCell<FtdiDevice>>,
}

impl SPI {
    pub fn new(ftdi: Rc<RefCell<FtdiDevice>>) -> SPI {
        SPI { ftdi: ftdi }
    }
}

impl hal::blocking::spi::Transfer<u8> for SPI {
    type Error = Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8]> {
        self.ftdi.borrow_mut().transfer(buffer)
    }
}

//

fn main() {
    let dev = FtdiDevice::spi_init(0x0403, 0x6014, None).unwrap();
    let dev_ref = Rc::new(RefCell::new(dev));
    let proxy1 = Rc::clone(&dev_ref);
    let proxy2 = Rc::clone(&dev_ref);
    let proxy3 = Rc::clone(&dev_ref);

    let ce = CE::new(proxy1);
    let cs = CS::new(proxy2);
    let spi = SPI::new(proxy3);

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

    // nRF24L01 simple tests
    let channels: Vec<u8> = vec![1, 2, 20, 40, 80, 100];
    for r in channels {
        nrf.set_frequency(r).unwrap();
        let ch = nrf.get_frequency().unwrap();
        println!("channel: {:?}", ch);
        assert_eq!(ch, r);
    }
}
