use adc_mcp3008::*;
use ftdi_embedded_hal as hal;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    #[cfg(all(feature = "ftdi-lib", feature = "ftd2-lib"))]
    compile_error!("features 'ftdi-lib' and 'ftd2-lib' cannot be enabled at the same time");

    #[cfg(not(any(feature = "ftdi-lib", feature = "ftd2-lib")))]
    compile_error!("one of features 'ftdi-lib' and 'ftd2-lib' shall be enabled");

    #[cfg(feature = "ftdi-lib")]
    let device = {
        let mut d = ftdi::find_by_vid_pid(0x0403, 0x6014)
            .interface(ftdi::Interface::A)
            .open()
            .unwrap();

        // TODO: set clocks in mpsse_init
        d.set_mpsse_clock(ftdi::MpsseClock::CLK_1MHz).unwrap();
        d
    };

    #[cfg(feature = "ftd2-lib")]
    let device = libftd2xx::Ft232h::with_description("Single RS232-HS").unwrap();

    let hal = hal::x232h::FTx232H::init(device, 1_000_000).unwrap();
    let spi = hal.spi().unwrap();
    let ncs = hal.pl2().unwrap();

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
