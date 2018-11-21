extern crate ftdi_embedded_hal as hal;

fn main() {
    let mut dev = hal::devices::um232_init(0x0403, 0x6014, false).unwrap();
    let regs: Vec<u8> = vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];

    hal::gpio::um232_spi_csn(&mut dev, 0x20, 0x0).unwrap();

    for r in regs {
        hal::gpio::um232_spi_csn(&mut dev, 0x40, 0x0).unwrap();
        let regval = hal::spi::um232_spi_byte_xfer(&mut dev, 0x00 | (0x1F & r)).unwrap();
        hal::gpio::um232_spi_csn(&mut dev, 0x40, 0x1).unwrap();
        println!("REG[0x{:x}] = [{:08b}]", r, regval);
    }
}
