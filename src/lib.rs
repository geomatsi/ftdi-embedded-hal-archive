extern crate ftdi;

use ftdi::BitMode;
use ftdi::FlowControl;

use std::io::{Read, Result, Write};

/* MPSSE commands */
#[allow(non_camel_case_types)]
pub enum MPSSECmd {
    SET_BITS_LOW,
    SET_BITS_HIGH,
    GET_BITS_LOW,
    GET_BITS_HIGH,
    LOOPBACK_START,
    LOOPBACK_END,
    TCK_DIVISOR,
}

impl Into<u8> for MPSSECmd {
    fn into(self) -> u8 {
        let cmd = match self {
            MPSSECmd::SET_BITS_LOW => 0x80,
            MPSSECmd::SET_BITS_HIGH => 0x82,
            MPSSECmd::GET_BITS_LOW => 0x81,
            MPSSECmd::GET_BITS_HIGH => 0x83,
            MPSSECmd::LOOPBACK_START => 0x84,
            MPSSECmd::LOOPBACK_END => 0x85,
            MPSSECmd::TCK_DIVISOR => 0x86,
        };

        cmd as u8
    }
}

/* H Type specific MPSSE commands */
#[allow(non_camel_case_types)]
pub enum MPSSECmd_H {
    DIS_DIV_5,
    EN_DIV_5,
    EN_3_PHASE,
    DIS_3_PHASE,
    CLK_BITS,
    CLK_BYTES,
    CLK_WAIT_HIGH,
    CLK_WAIT_LOW,
    EN_ADAPTIVE,
    DIS_ADAPTIVE,
    CLK_BYTES_OR_HIGH,
    CLK_BYTES_OR_LOW,
}

impl Into<u8> for MPSSECmd_H {
    fn into(self) -> u8 {
        let cmd = match self {
            MPSSECmd_H::DIS_DIV_5 => 0x8a,
            MPSSECmd_H::EN_DIV_5 => 0x8b,
            MPSSECmd_H::EN_3_PHASE => 0x8c,
            MPSSECmd_H::DIS_3_PHASE => 0x8d,
            MPSSECmd_H::CLK_BITS => 0x8e,
            MPSSECmd_H::CLK_BYTES => 0x8f,
            MPSSECmd_H::CLK_WAIT_HIGH => 0x94,
            MPSSECmd_H::CLK_WAIT_LOW => 0x95,
            MPSSECmd_H::EN_ADAPTIVE => 0x96,
            MPSSECmd_H::DIS_ADAPTIVE => 0x97,
            MPSSECmd_H::CLK_BYTES_OR_HIGH => 0x9c,
            MPSSECmd_H::CLK_BYTES_OR_LOW => 0x9d,
        };

        cmd as u8
    }
}

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
