use std::cell::RefCell;
use std::io::{Error, Result};
use std::sync::Mutex;

pub struct I2cBus<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Context>>,
    speed: u32,
}

impl<'a> I2cBus<'a> {
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Context>>) -> I2cBus {
        I2cBus { ctx: ctx, speed: 0 }
    }

    pub fn speed(mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn get_speed(self) -> u32 {
        return self.speed;
    }
}

impl<'a> hal::blocking::i2c::Read for I2cBus<'a> {
    type Error = Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<()> {
        Ok(())
    }
}

impl<'a> hal::blocking::i2c::Write for I2cBus<'a> {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<()> {
        Ok(())
    }
}

impl<'a> hal::blocking::i2c::WriteRead for I2cBus<'a> {
    type Error = Error;

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<()> {
        Ok(())
    }
}
