use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin};
use ftdi_embedded_hal as hal;
use hal::x232h::FTx232H;

fn main() {
    let regs: Vec<u8> = vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];

    let dev = FTx232H::init(0x0403, 0x6014).unwrap();
    let mut spidev = dev.spi(hal::spi::SpiSpeed::CLK_1MHz).unwrap();
    let mut pl2 = dev.pl2().unwrap();

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
