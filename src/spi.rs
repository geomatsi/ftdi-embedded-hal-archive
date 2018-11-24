pub use hal::spi::{Mode, Phase, Polarity};

use crate::devices::FtdiDevice;
use std::io::{Error, Read, Result, Write};

fn len2cmd(sz: usize) -> (u8, u8) {
    let sl: u8 = ((sz - 1) & 0xff) as u8;
    let sh: u8 = (((sz - 1) >> 8) & 0xff) as u8;

    (sl, sh)
}

impl hal::blocking::spi::Transfer<u8> for FtdiDevice {
    type Error = Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8]> {
        if buffer.len() < 1 {
            return Ok(buffer);
        }

        let (sl, sh) = len2cmd(buffer.len());
        let mut cmd: Vec<u8> = vec![0x31, sl, sh];
        cmd.append(&mut buffer.to_vec());

        self.ctx.usb_purge_buffers()?;
        self.ctx.write_all(&mut cmd)?;
        self.ctx.read_exact(buffer)?;

        Ok(buffer)
    }
}

impl hal::blocking::spi::Write<u8> for FtdiDevice {
    type Error = Error;

    fn write(&mut self, buffer: &[u8]) -> Result<()> {
        if buffer.len() < 1 {
            return Ok(());
        }

        let (sl, sh) = len2cmd(buffer.len());
        let mut cmd: Vec<u8> = vec![0x20, sl, sh];
        cmd.append(&mut buffer.to_vec());

        self.ctx.usb_purge_buffers()?;
        self.ctx.write_all(&mut cmd)
    }
}
