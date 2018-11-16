extern crate ftdi;

use ftdi::BitMode;
use ftdi::FlowControl;
use ftdi::MPSSECmd;
use ftdi::MPSSECmd_H;

use std::io::{Read, Result, Write};
use std::process;

fn um232_init(loopback: bool) -> Result<ftdi::Context> {
    let mut context = ftdi::Context::new();

    if !context.usb_open(0x0403, 0x6014).is_ok() {
        println!("No FTDI device...");
        process::exit(-1);
    }

    context.set_write_chunksize(32);
    context.set_interface(ftdi::Interface::A)?;
    context.usb_reset()?;
    context.set_latency_timer(2)?;
    context.set_flow_control(FlowControl::SIO_RTS_CTS_HS)?;
    context.set_bitmode(0, BitMode::MPSSE)?;
    context.usb_purge_buffers()?;

    // init gpio to high
    context.write_all(&vec![MPSSECmd::SET_BITS_LOW.into(), 0x0, 0xfb])?;
    context.write_all(&vec![MPSSECmd::SET_BITS_HIGH.into(), 0x0, 0xff])?;

    // configure loopback
    let cmd = if loopback {
        MPSSECmd::LOOPBACK_START
    } else {
        MPSSECmd::LOOPBACK_END
    };
    //context.write_all(&vec![0x85]).expect("failed to configure loopback");
    context.write_all(&vec![cmd.into()])?;

    // set speed
    context.write_all(&vec![MPSSECmd_H::EN_DIV_5.into()])?;
    context
        .write_all(&vec![MPSSECmd::TCK_DIVISOR.into(), 59, 0])
        .expect("failed to configure clock speed");

    Ok(context)
}

fn main() {
    // test #1: loopback
    {
        let mut context = um232_init(true).unwrap();
        let vals = vec![0x0, 0x1, 0x10, 0x20];

        for v in vals {
            let mut rsp: Vec<u8> = vec![0];

            context.usb_purge_buffers().unwrap();
            context.write_all(&vec![0x31, 0x0, 0x0, v]).unwrap();
            context.read_exact(&mut rsp).unwrap();
            println!("{} {}", v, rsp[0]);
        }
    }

    // test #2: nRF24 regs
    {
        let mut context = um232_init(false).unwrap();
        let regs: Vec<u8> = vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9];

        um232_spi_csn(&mut context, 0x20, 0x0);

        for r in regs {
            um232_spi_csn(&mut context, 0x40, 0x0);
            let (status, regval) = um232_spi_byte_xfer(&mut context, 0x00 | (0x1F & r));
            um232_spi_csn(&mut context, 0x40, 0x1);
            println!("REG[{:x}] -> STATUS[{:b}] REGVAL[{:b}]", r, status, regval);
        }
    }
}

fn um232_spi_byte_xfer(ctx: &mut ftdi::Context, data: u8) -> (u8, u8) {
    let mut rsp: Vec<u8> = vec![0];

    ctx.usb_purge_buffers().expect("failed to purge buffers");
    ctx.write_all(&vec![0x31, 0x0, 0x0, data])
        .expect("failed to write byte");
    ctx.read_exact(&mut rsp).expect("failed to read byte");
    let status = rsp[0];

    ctx.usb_purge_buffers().expect("failed to purge buffers");
    ctx.write_all(&vec![0x31, 0x0, 0x0, 0xff])
        .expect("failed to write byte");
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
