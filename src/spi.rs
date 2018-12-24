pub use embedded_hal::spi::{Mode, Phase, Polarity};
pub use embedded_hal::spi::{MODE_0, MODE_1, MODE_2, MODE_3};

use crate::mpsse::MPSSECmd;

use std::cell::RefCell;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::sync::Mutex;

pub struct SpiBus<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Context>>,
    mode: Mode,
    speed: u32,
    cmd_rw: MPSSECmd,
    cmd_w: MPSSECmd,
}

impl<'a> SpiBus<'a> {
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Context>>) -> SpiBus {
        SpiBus {
            ctx,
            speed: 0,
            mode: MODE_0,
            cmd_rw: MPSSECmd::MSB_BYTES_RW_CPOL_0_CPHA_0,
            cmd_w: MPSSECmd::MSB_BYTES_W_CPOL_0_CPHA_0,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<()> {
        if mode == MODE_0 {
            self.cmd_rw = MPSSECmd::MSB_BYTES_RW_CPOL_0_CPHA_0;
            self.cmd_w = MPSSECmd::MSB_BYTES_W_CPOL_0_CPHA_0;
            self.mode = mode;
            return Ok(())
        }

        if mode == MODE_2 {
            self.cmd_rw = MPSSECmd::MSB_BYTES_RW_CPOL_1_CPHA_0;
            self.cmd_w = MPSSECmd::MSB_BYTES_W_CPOL_1_CPHA_0;
            self.mode = mode;
            return Ok(())
        }

        Err(Error::new(ErrorKind::NotFound, "mode not supported"))
    }

    pub fn get_mode(&mut self) -> Mode {
        self.mode
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn get_speed(&mut self) -> u32 {
        self.speed
    }
}

impl<'a> SpiBus<'a> {
    fn len2cmd(sz: usize) -> (u8, u8) {
        let sl: u8 = ((sz - 1) & 0xff) as u8;
        let sh: u8 = (((sz - 1) >> 8) & 0xff) as u8;

        (sl, sh)
    }
}

impl<'a> embedded_hal::blocking::spi::Transfer<u8> for SpiBus<'a> {
    type Error = Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8]> {
        if buffer.is_empty() {
            return Ok(buffer);
        }

        let (sl, sh) = SpiBus::len2cmd(buffer.len());
        let mut cmd: Vec<u8> = vec![];

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        cmd.append(&mut vec![self.cmd_rw.into(), sl, sh]);
        cmd.append(&mut buffer.to_vec());
        cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;
        ftdi.read_exact(buffer)?;

        Ok(buffer)
    }
}

impl<'a> embedded_hal::blocking::spi::Write<u8> for SpiBus<'a> {
    type Error = Error;

    fn write(&mut self, buffer: &[u8]) -> Result<()> {
        if buffer.is_empty() {
            return Ok(());
        }

        let (sl, sh) = SpiBus::len2cmd(buffer.len());
        let mut cmd: Vec<u8> = vec![];

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        cmd.append(&mut vec![self.cmd_w.into(), sl, sh]);
        cmd.append(&mut buffer.to_vec());
        cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)
    }
}
