use std::io::{Read, Result, Write};

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


