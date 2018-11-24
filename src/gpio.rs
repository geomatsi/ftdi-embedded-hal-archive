use crate::mpsse::MPSSECmd;
use std::io::{Read, Result, Write};

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
