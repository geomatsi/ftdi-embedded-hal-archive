extern crate ftdi;

use ftdi::BitMode;
use ftdi::FlowControl;
use ftdi::MPSSECmd;
use ftdi::MPSSECmd_H;

use std::io::{Read, Result, Write};

pub fn um232_init(vendor: u16, product: u16, loopback: bool) -> Result<ftdi::Context> {
    let mut context = ftdi::Context::new();

    if !context.usb_open(vendor, product).is_ok() {
        panic!("No FTDI device");
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

    context.write_all(&vec![cmd.into()])?;

    // set speed
    context.write_all(&vec![MPSSECmd_H::EN_DIV_5.into()])?;
    context.write_all(&vec![MPSSECmd::TCK_DIVISOR.into(), 59, 0])?;

    Ok(context)
}

pub fn um232_spi_byte_xfer(ctx: &mut ftdi::Context, data: u8) -> Result<u8> {
    let mut rsp: Vec<u8> = vec![0];

    // FIXME: 0x31 - MPSSE/FT232H specific command ?
    ctx.usb_purge_buffers()?;
    ctx.write_all(&vec![0x31, 0x0, 0x0, data])?;
    ctx.read_exact(&mut rsp)?;

    ctx.usb_purge_buffers()?;
    ctx.write_all(&vec![0x31, 0x0, 0x0, 0xff])?;
    ctx.read_exact(&mut rsp)?;

    Ok(rsp[0])
}

pub fn um232_spi_csn(ctx: &mut ftdi::Context, bit: u8, level: u8) -> Result<()> {
    let mut cmd: Vec<u8> = vec![MPSSECmd::SET_BITS_LOW.into(), 0x0, 0xfb];
    let mut pins: Vec<u8> = vec![0];

    ctx.usb_purge_buffers()?;

    ctx.write_all(&vec![MPSSECmd::GET_BITS_LOW.into()])?;
    ctx.read_exact(&mut pins)?;

    if level > 0 {
        cmd[1] = pins[0] | bit;
    } else {
        cmd[1] = pins[0] & (!bit);
    }

    ctx.write_all(&cmd)?;

    Ok(())
}
