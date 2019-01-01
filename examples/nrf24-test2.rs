extern crate ftdi_embedded_hal as hal;
use crate::hal::x232h::FTx232H;

extern crate embedded_nrf24l01;
use embedded_nrf24l01::Configuration;
use embedded_nrf24l01::CrcMode;
use embedded_nrf24l01::DataRate;
use embedded_nrf24l01::NRF24L01;

fn main() {
    let dev = FTx232H::init(0x0403, 0x6014).unwrap();
    let spidev = dev.spi(hal::spi::SpiSpeed::CLK_1MHz).unwrap();
    let cs = dev.pl2().unwrap();
    let ce = dev.ph0().unwrap();

    // nRF24L01 setup
    let mut nrf = NRF24L01::new(ce, cs, spidev).unwrap();
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
