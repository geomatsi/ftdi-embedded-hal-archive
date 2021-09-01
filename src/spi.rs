pub use embedded_hal::spi::{Mode, Phase, Polarity};
pub use embedded_hal::spi::{MODE_0, MODE_1, MODE_2, MODE_3};

use crate::error::{ErrorKind, Result, X232Error};

use ftdi_mpsse::{ClockData, ClockDataIn, ClockDataOut, MpsseCmdBuilder, MpsseCmdExecutor};
use nb;
use std::cell::RefCell;
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
pub struct SpiBus<'a, T>
where
    T: MpsseCmdExecutor,
    X232Error: From<<T as MpsseCmdExecutor>::Error>,
{
    ctx: &'a Mutex<RefCell<T>>,
    mode: Mode,
    cmd_r: ClockDataIn,
    cmd_w: ClockDataOut,
    cmd_rw: ClockData,
}

impl<'a, T> SpiBus<'a, T>
where
    T: MpsseCmdExecutor,
    X232Error: From<<T as MpsseCmdExecutor>::Error>,
{
    pub fn new(ctx: &'a Mutex<RefCell<T>>) -> SpiBus<T> {
        SpiBus {
            ctx,
            mode: MODE_0,
            cmd_r: ClockDataIn::MsbPos,
            cmd_w: ClockDataOut::MsbNeg,
            // cmd_rw = cmd_r | cmd_w
            cmd_rw: ClockData::MsbPosIn,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<()> {
        if mode == MODE_0 {
            self.cmd_r = ClockDataIn::MsbPos;
            self.cmd_w = ClockDataOut::MsbNeg;
            // cmd_rw = cmd_r | cmd_w
            self.cmd_rw = ClockData::MsbPosIn;
            self.mode = mode;
            return Ok(());
        }

        if mode == MODE_2 {
            self.cmd_r = ClockDataIn::MsbNeg;
            self.cmd_w = ClockDataOut::MsbPos;
            // cmd_rw = cmd_r | cmd_w
            self.cmd_rw = ClockData::MsbNegIn;
            self.mode = mode;
            return Ok(());
        }

        Err(X232Error::HAL(ErrorKind::SpiModeNotSupported))
    }

    pub fn get_mode(&mut self) -> Mode {
        self.mode
    }
}

impl<'a, T> embedded_hal::blocking::spi::Transfer<u8> for SpiBus<'a, T>
where
    T: MpsseCmdExecutor,
    X232Error: From<<T as MpsseCmdExecutor>::Error>,
{
    type Error = X232Error;

    fn transfer<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b [u8]> {
        if buffer.is_empty() {
            return Ok(buffer);
        }

        let cmd: MpsseCmdBuilder = MpsseCmdBuilder::new()
            .clock_data(self.cmd_rw, buffer)
            .send_immediate();

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.mpsse_xfer(cmd.as_slice(), buffer)?;

        Ok(buffer)
    }
}

impl<'a, T> embedded_hal::blocking::spi::Write<u8> for SpiBus<'a, T>
where
    T: MpsseCmdExecutor,
    X232Error: From<<T as MpsseCmdExecutor>::Error>,
{
    type Error = X232Error;

    fn write(&mut self, buffer: &[u8]) -> Result<()> {
        if buffer.is_empty() {
            return Ok(());
        }

        let cmd: MpsseCmdBuilder = MpsseCmdBuilder::new()
            .clock_data_out(self.cmd_w, buffer)
            .send_immediate();

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.mpsse_send(cmd.as_slice())?;

        Ok(())
    }
}

impl<'a, T> embedded_hal::spi::FullDuplex<u8> for SpiBus<'a, T>
where
    T: MpsseCmdExecutor,
    X232Error: From<<T as MpsseCmdExecutor>::Error>,
{
    type Error = X232Error;

    fn read(&mut self) -> nb::Result<u8, X232Error> {
        let mut buffer: [u8; 1] = [0];

        let cmd: MpsseCmdBuilder = MpsseCmdBuilder::new()
            .clock_data(self.cmd_rw, &buffer)
            .send_immediate();

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        match ftdi.mpsse_xfer(cmd.as_slice(), &mut buffer) {
            Ok(()) => Ok(buffer[0]),
            Err(e) => Err(nb::Error::Other(X232Error::from(e))),
        }
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), X232Error> {
        let cmd: MpsseCmdBuilder = MpsseCmdBuilder::new()
            .clock_data_out(self.cmd_w, &[byte])
            .send_immediate();

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        match ftdi.mpsse_send(cmd.as_slice()) {
            Ok(()) => Ok(()),
            Err(e) => Err(nb::Error::Other(X232Error::from(e))),
        }
    }
}
