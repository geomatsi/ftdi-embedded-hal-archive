#![feature(extern_crate_item_prelude)]

extern crate embedded_hal as hal;
extern crate ftdi;
extern crate nb;

pub mod ft232h;
pub mod gpio;
pub mod mpsse;
pub mod spi;

//

#[cfg(test)]
mod test {
    use super::ft232h::FT232H;

    #[test]
    fn test_init_t1() {
        let mut dev = FT232H::init(0x0403, 0x6014).unwrap();
        assert_eq!(dev.is_loopback(), false);

        dev.loopback(true).unwrap();
        assert_eq!(dev.is_loopback(), true);

        let spidev = dev.spi().unwrap();
        assert_eq!(spidev.get_speed(), 0);
    }
}
