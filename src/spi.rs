pub use hal::spi::{MODE_0, Mode, Phase, Polarity};

use std::cell::RefCell;
use std::io::{Error, Read, Result, Write};
use std::sync::Mutex;

pub struct SpiBus<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Context>>,
    mode: Mode,
    speed: u32,
}

impl<'a> SpiBus<'a> {
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Context>>) -> SpiBus {
        SpiBus {
            ctx: ctx,
            mode: MODE_0,
            speed: 0,
        }
    }

    pub fn mode(mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn get_mode(self) -> Mode {
        return self.mode;
    }

    pub fn speed(mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn get_speed(self) -> u32 {
        return self.speed;
    }
}

impl<'a> SpiBus<'a> {
    fn len2cmd(sz: usize) -> (u8, u8) {
        let sl: u8 = ((sz - 1) & 0xff) as u8;
        let sh: u8 = (((sz - 1) >> 8) & 0xff) as u8;

        (sl, sh)
    }
}

impl<'a> hal::blocking::spi::Transfer<u8> for SpiBus<'a> {
    type Error = Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8]> {
        if buffer.len() < 1 {
            return Ok(buffer);
        }

        let (sl, sh) = SpiBus::len2cmd(buffer.len());
        let mut cmd: Vec<u8> = vec![0x31, sl, sh];
        cmd.append(&mut buffer.to_vec());

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&mut cmd)?;
        ftdi.read_exact(buffer)?;

        Ok(buffer)
    }
}

impl<'a> hal::blocking::spi::Write<u8> for SpiBus<'a> {
    type Error = Error;

    fn write(&mut self, buffer: &[u8]) -> Result<()> {
        if buffer.len() < 1 {
            return Ok(());
        }

        let (sl, sh) = SpiBus::len2cmd(buffer.len());
        let mut cmd: Vec<u8> = vec![0x20, sl, sh];
        cmd.append(&mut buffer.to_vec());

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&mut cmd)
    }
}
