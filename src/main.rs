extern crate ftdi;

use ftdi::BitMode;
use ftdi::FlowControl;

use std::io::{Read, Write};
use std::process;

fn main() {
    let mut context = ftdi::Context::new();
    context.set_interface(ftdi::Interface::A).expect("failed to set interface");

    if !context.usb_open(0x0403, 0x6014).is_ok() {
        process::exit(-1);
    }

    context.set_write_chunksize(32);
    context.usb_reset().expect("failed to reset usb");
    context.usb_purge_buffers().expect("failed to purge buffers");
    context.set_latency_timer(2).expect("failed to set latency timer");
    context.set_flow_control(FlowControl::SIO_RTS_CTS_HS).expect("failed to configure flow control");
    context.set_bitmode(0, BitMode::MPSSE).expect("failed to set bitmode");

    // disable loopback
    context.write_all(&vec![0x85]).expect("failed to configure loopback");

    // set speed
    context.write_all(&vec![0x8b]).expect("failed to configure clock div");
    context.write_all(&vec![0x86, 60, 0]).expect("failed to configure clock speed");

    // xfer byte over spi
    context.write_all(&vec![0x31, 0x0, 0x0, 0x01]).expect("failed to write byte");
    let mut rsp = vec![0];
    context.read_exact(&mut rsp).expect("failed to read byte");

    // TODO: CE/CSN GPIOs + SPI read register command
}
