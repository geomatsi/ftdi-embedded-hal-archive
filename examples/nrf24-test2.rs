extern crate ftdi_embedded_hal as hal;
use crate::hal::x232h::FTx232H;

extern crate embedded_nrf24l01;
use embedded_nrf24l01::Configuration;
use embedded_nrf24l01::CrcMode;
use embedded_nrf24l01::DataRate;
use embedded_nrf24l01::NRF24L01;

use std::thread::sleep;
use std::time::Duration;

// Simple Tx test for embedded-nrf24l01 crate

fn main() {
    let dev = FTx232H::init(0x0403, 0x6014).unwrap();
    let spidev = dev.spi(hal::spi::SpiSpeed::CLK_1MHz).unwrap();
    let ce = dev.pl1().unwrap();
    let cs = dev.pl2().unwrap();

    // nRF24L01 setup
    let mut nrf = NRF24L01::new(ce, cs, spidev).unwrap();
    nrf.set_frequency(120).unwrap();
    nrf.set_rf(&DataRate::R250Kbps, 3 /* 0 dBm */).unwrap();
    nrf.set_crc(CrcMode::OneByte).unwrap();
    nrf.set_auto_retransmit(0b0100, 0b1111).unwrap();

    let addr: [u8; 5] = [0xe5, 0xe4, 0xe3, 0xe2, 0xe1];
    nrf.set_rx_addr(0, &addr).unwrap();
    nrf.set_tx_addr(&addr).unwrap();
    nrf.set_pipes_rx_lengths(&[None; 6]).unwrap();
    nrf.flush_tx().unwrap();
    nrf.flush_rx().unwrap();

    let delay = Duration::from_millis(1000);
    let mut tx = nrf.tx().unwrap();
    let msg = b"hello";

    sleep(delay);

    loop {
        println!("Tx: {:?}", msg);
        tx.send(msg).unwrap();
        tx.wait_empty().unwrap();

        sleep(delay);
    }
}
