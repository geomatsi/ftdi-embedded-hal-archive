use embedded_nrf24l01::Configuration;
use embedded_nrf24l01::CrcMode;
use embedded_nrf24l01::DataRate;
use embedded_nrf24l01::NRF24L01;
use ftdi_embedded_hal as hal;
use std::thread::sleep;
use std::time::Duration;

// Simple Rx test for embedded-nrf24l01 crate

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
    let spidev = hal.spi().unwrap();
    let ce = hal.pl1().unwrap();
    let cs = hal.pl2().unwrap();

    let mut nrf = NRF24L01::new(ce, cs, spidev).unwrap();
    nrf.set_frequency(120).unwrap();
    nrf.set_rf(&DataRate::R250Kbps, 3 /* 0 dBm */).unwrap();
    nrf.set_crc(CrcMode::OneByte).unwrap();
    nrf.set_auto_retransmit(0b0100, 0b1111).unwrap();
    nrf.set_auto_ack(&[true; 6]).unwrap();

    let addr: [u8; 5] = [0xe5, 0xe4, 0xe3, 0xe2, 0xe1];
    nrf.set_rx_addr(0, &addr).unwrap();
    nrf.set_pipes_rx_lengths(&[None; 6]).unwrap();
    nrf.set_pipes_rx_enable(&[true, false, false, false, false, false])
        .unwrap();
    nrf.flush_tx().unwrap();
    nrf.flush_rx().unwrap();

    let delay = Duration::from_millis(500);
    let mut rx = nrf.rx().unwrap();

    loop {
        let pipe = rx.can_read().unwrap();

        if pipe.is_some() {
            let data = rx.read().unwrap();
            println!("Rx: {:?}", data.as_ref());
        }

        sleep(delay);
    }
}
