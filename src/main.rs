extern crate ftdi;

use ftdi::BitMode;
use ftdi::FlowControl;

use std::io::{Read, Write};
use std::{thread, time};
use std::process;

fn main() {
    let mut context = ftdi::Context::new();

    if !context.usb_open(0x0403, 0x6014).is_ok() {
        println!("No FTDI device...");
        process::exit(-1);
    }

    context.set_write_chunksize(32);
    context.set_interface(ftdi::Interface::A).unwrap();
    context.usb_reset().unwrap();
    context.set_latency_timer(2).unwrap();
    context.set_flow_control(FlowControl::SIO_RTS_CTS_HS).unwrap();
    context.set_bitmode(0, BitMode::MPSSE).unwrap();
    context.usb_purge_buffers().unwrap();

    // init gpio to high
    context.write_all(&vec![0x80, 0x0, 0xfb]).unwrap();
    context.write_all(&vec![0x82, 0x0, 0xff]).unwrap();

    // disable loopback
    context.write_all(&vec![0x85]).expect("failed to configure loopback");

    // set speed
    context.write_all(&vec![0x8b]).expect("failed to configure clock div");
    context.write_all(&vec![0x86, 59, 0]).expect("failed to configure clock speed");

    // read nRF24 registers
    let regs: Vec<u8> = vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];

    um232_spi_csn(&mut context, 0x20, 0x0);

    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);

    for r in regs {
        um232_spi_csn(&mut context, 0x40, 0x0);
        let (status, regval) = um232_spi_byte_xfer(&mut context, 0x00 | (0x1F & r));
        um232_spi_csn(&mut context, 0x40, 0x1);
        println!("REG[{:x}] -> STATUS[{:b}] REGVAL[{:b}]", r, status, regval);
    }
}

fn um232_spi_byte_xfer(ctx: &mut ftdi::Context, data: u8) -> (u8, u8) {
    let mut rsp: Vec<u8> = vec![0];

    ctx.usb_purge_buffers().expect("failed to purge buffers");
    ctx.write_all(&vec![0x31, 0x0, 0x0, data]).expect("failed to write byte");
    ctx.read_exact(&mut rsp).expect("failed to read byte");
    let status = rsp[0];

    ctx.usb_purge_buffers().expect("failed to purge buffers");
    ctx.write_all(&vec![0x31, 0x0, 0x0, 0xff]).expect("failed to write byte");
    ctx.read_exact(&mut rsp).expect("failed to read byte");
    let regval = rsp[0];

    (status, regval)
}

fn um232_spi_csn(ctx: &mut ftdi::Context, bit: u8, level: u8) {
    let mut cmd: Vec<u8> = vec![0x80, 0x0, 0xfb];
    let mut pins: Vec<u8> = vec![0];

    ctx.usb_purge_buffers().expect("failed to purge buffers");

    ctx.write_all(&vec![0x81]).expect("failed to write byte");
    ctx.read_exact(&mut pins).expect("failed to read byte");

    if level > 0 {
        cmd[1] = pins[0] | bit;
    } else {
        cmd[1] = pins[0] & (!bit);
    }

    ctx.write_all(&cmd).expect("failed to write");
}
