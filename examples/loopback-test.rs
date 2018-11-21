extern crate ftdi_embedded_hal as hal;

use std::io::{Read, Write};

fn main() {
    let mut dev = hal::devices::um232_init(0x0403, 0x6014, true).unwrap();

    for v in 0x0..0xff {
        let mut rsp: Vec<u8> = vec![0];

        dev.usb_purge_buffers().unwrap();
        dev.write_all(&vec![0x31, 0x0, 0x0, v]).unwrap();
        dev.read_exact(&mut rsp).unwrap();
        assert_eq!(v, rsp[0]);
    }

    println!("Loopback ok!");
}
