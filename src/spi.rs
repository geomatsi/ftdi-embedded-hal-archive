pub use embedded_hal::spi::{Mode, Phase, Polarity};
pub use embedded_hal::spi::{MODE_0, MODE_1, MODE_2, MODE_3};

use crate::error::{X232Error, Result, ErrorKind};
use crate::mpsse::MPSSECmd;

use nb;

use std::cell::RefCell;
use std::io::{Read, Write};
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SpiSpeed {
    CLK_AUTO,
    CLK_500kHz,
    CLK_1MHz,
    CLK_2MHz,
    CLK_2_5MHz,
    CLK_3MHz,
    CLK_5MHz,
    CLK_10MHz,
    CLK_20MHz,
}

pub struct SpiBus<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Device>>,
    mode: Mode,
    cmd_w: MPSSECmd,
    cmd_r: MPSSECmd,
}

impl<'a> SpiBus<'a> {
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Device>>) -> SpiBus {
        SpiBus {
            ctx,
            mode: MODE_0,
            cmd_r: MPSSECmd::MSB_RISING_EDGE_CLK_BYTE_IN,
            cmd_w: MPSSECmd::MSB_FALLING_EDGE_CLK_BYTE_OUT,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<()> {
        if mode == MODE_0 {
            self.cmd_r = MPSSECmd::MSB_RISING_EDGE_CLK_BYTE_IN;
            self.cmd_w = MPSSECmd::MSB_FALLING_EDGE_CLK_BYTE_OUT;
            self.mode = mode;
            return Ok(());
        }

        if mode == MODE_2 {
            self.cmd_r = MPSSECmd::MSB_FALLING_EDGE_CLK_BYTE_IN;
            self.cmd_w = MPSSECmd::MSB_RISING_EDGE_CLK_BYTE_OUT;
            self.mode = mode;
            return Ok(());
        }

        Err(X232Error::HAL(ErrorKind::SpiModeNotSupported))
    }

    pub fn get_mode(&mut self) -> Mode {
        self.mode
    }
}

impl<'a> SpiBus<'a> {
    fn len2cmd(sz: usize) -> (u8, u8) {
        let sl: u8 = ((sz - 1) & 0xff) as u8;
        let sh: u8 = (((sz - 1) >> 8) & 0xff) as u8;

        (sl, sh)
    }

    fn cmd_rw(&self) -> u8 {
        let a: u8 = self.cmd_r.into();
        let b: u8 = self.cmd_w.into();
        a | b
    }
}

impl<'a> embedded_hal::blocking::spi::Transfer<u8> for SpiBus<'a> {
    type Error = X232Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8]> {
        if buffer.is_empty() {
            return Ok(buffer);
        }

        let (sl, sh) = SpiBus::len2cmd(buffer.len());
        let mut cmd: Vec<u8> = vec![];

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        cmd.append(&mut vec![self.cmd_rw(), sl, sh]);
        cmd.append(&mut buffer.to_vec());
        cmd.append(&mut vec![MPSSECmd::SEND_IMMEDIATE_RESP.into()]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;
        ftdi.read_exact(buffer)?;

        Ok(buffer)
    }
}

impl<'a> embedded_hal::blocking::spi::Write<u8> for SpiBus<'a> {
    type Error = X232Error;

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
        cmd.append(&mut vec![MPSSECmd::SEND_IMMEDIATE_RESP.into()]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;

        Ok(())
    }
}

impl<'a> embedded_hal::spi::FullDuplex<u8> for SpiBus<'a> {
    type Error = X232Error;

    fn read(&mut self) -> nb::Result<u8, X232Error> {
        let mut buffer: [u8; 1] = [0];

        let (sl, sh) = SpiBus::len2cmd(buffer.len());
        let mut cmd: Vec<u8> = vec![];

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        cmd.append(&mut vec![self.cmd_rw(), sl, sh]);
        cmd.append(&mut buffer.to_vec());
        cmd.append(&mut vec![MPSSECmd::SEND_IMMEDIATE_RESP.into()]);

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&cmd).unwrap();
        ftdi.read_exact(&mut buffer).unwrap();

        Ok(buffer[0])
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), X232Error> {
        let (sl, sh) = SpiBus::len2cmd(1);
        let mut cmd: Vec<u8> = vec![];

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        cmd.append(&mut vec![self.cmd_w.into(), sl, sh]);
        cmd.append(&mut vec![byte]);
        cmd.append(&mut vec![MPSSECmd::SEND_IMMEDIATE_RESP.into()]);

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&cmd).unwrap();

        Ok(())
    }
}