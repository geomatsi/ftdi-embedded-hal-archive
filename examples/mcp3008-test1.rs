use adc_mcp3008::*;
use ftdi_embedded_hal as hal;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let dev = hal::x232h::FTx232H::init(0x0403, 0x6014).unwrap();
    let spi = dev.spi(hal::spi::SpiSpeed::CLK_1MHz).unwrap();
    let ncs = dev.pl2().unwrap();

    let mut adc = Mcp3008::new(spi, ncs).unwrap();

    loop {
        println!("CH0: {:?}", adc.read_channel(Channels8::CH0));
        println!("CH1: {:?}", adc.read_channel(Channels8::CH1));
        println!("CH2: {:?}", adc.read_channel(Channels8::CH2));
        println!("CH3: {:?}", adc.read_channel(Channels8::CH3));
        println!("CH4: {:?}", adc.read_channel(Channels8::CH4));
        println!("CH5: {:?}", adc.read_channel(Channels8::CH5));
        println!("CH6: {:?}", adc.read_channel(Channels8::CH6));
        println!("CH7: {:?}", adc.read_channel(Channels8::CH7));
        sleep(Duration::from_millis(1000));
    }
}
