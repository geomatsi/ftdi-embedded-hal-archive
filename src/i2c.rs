#![allow(clippy::identity_op)]

use crate::error::{ErrorKind, Result, X232Error};

use ftdi_mpsse::{ClockBitsIn, ClockDataIn, ClockDataOut, MpsseCmdBuilder};
use std::cell::RefCell;
use std::io::{Read, Write};
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum I2cSpeed {
    CLK_AUTO,
    CLK_100kHz,
    CLK_400kHz,
}

pub struct I2cBus<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Device>>,
}

impl<'a> I2cBus<'a> {
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Device>>) -> I2cBus {
        I2cBus { ctx }
    }
}

impl<'a> I2cBus<'a> {
    fn i2c_write_to(addr: u8) -> u8 {
        (addr << 1) | 0x0
    }

    fn i2c_read_from(addr: u8) -> u8 {
        (addr << 1) | 0x1
    }
}

impl<'a> I2cBus<'a> {
    fn i2c_start(&self, mut cmd: MpsseCmdBuilder, pins: u8) -> MpsseCmdBuilder {
        for _ in 0..4 {
            cmd = cmd.set_gpio_lower((pins & 0b1111_1000) | 0b11, 0b1111_1011);
        }

        for _ in 0..4 {
            cmd = cmd.set_gpio_lower((pins & 0b1111_1000) | 0b01, 0b1111_1011);
        }

        for _ in 0..4 {
            cmd = cmd.set_gpio_lower((pins & 0b1111_1000) | 0b00, 0b1111_1011);
        }

        cmd
    }

    fn i2c_stop(&self, mut cmd: MpsseCmdBuilder, pins: u8) -> MpsseCmdBuilder {
        for _ in 0..4 {
            cmd = cmd.set_gpio_lower((pins & 0b1111_1000) | 0b01, 0b1111_1011);
        }

        for _ in 0..4 {
            cmd = cmd.set_gpio_lower((pins & 0b1111_1000) | 0b11, 0b1111_1011);
        }

        for _ in 0..4 {
            cmd = cmd.set_gpio_lower((pins & 0b1111_1100) | 0b00, 0b1111_1000);
        }

        cmd
    }

    fn i2c_write_byte_ack(&self, cmd: MpsseCmdBuilder, byte: u8, pins: u8) -> MpsseCmdBuilder {
        cmd
            // make sure no occasional SP: SDA output(1) SCL output(0)
            .set_gpio_lower((pins & 0b1111_1000) | 0b10, 0b1111_1011)
            // send single byte using MPSSE
            .clock_data_out(ClockDataOut::MsbNeg, &[byte])
            // get pins ready for SAK: DO input, DI input, SK output(0)
            .set_gpio_lower((pins & 0b1111_1000) | 0b00, 0b1111_1001)
            // SAK: recv using MPSSE
            .clock_bits_in(ClockBitsIn::MsbPos, 1)
            // request immediate response from FTDI to host
            .send_immediate()
    }

    fn i2c_read_byte(&self, cmd: MpsseCmdBuilder, nack: bool, pins: u8) -> MpsseCmdBuilder {
        let state = if nack {
            (pins & 0b1111_1000) | 0b10
        } else {
            (pins & 0b1111_1000) | 0b00
        };

        cmd
            // make sure no occasional SP: SDA output(1), SCL output(0)
            .set_gpio_lower((pins & 0b1111_1000) | 0b10, 0b1111_1011)
            // prepare to read: SDA input, SCL output(0)
            .set_gpio_lower((pins & 0b1111_1000) | 0b00, 0b1111_1001)
            // read byte using MPSSE
            .clock_data_in(ClockDataIn::MsbNeg, 1)
            // prepare SDA for NACK/ACK
            .set_gpio_lower(state, 0b1111_1011)
            // NACK/ACK to slave: we pretend we read it
            .clock_bits_in(ClockBitsIn::MsbPos, 1)
            // request immediate response from FTDI to PC
            .send_immediate()
    }
}

impl<'a> embedded_hal::blocking::i2c::Read for I2cBus<'a> {
    type Error = X232Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<()> {
        if buffer.is_empty() {
            return Ok(());
        }

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        let cmd_read_low_pins = MpsseCmdBuilder::new().gpio_lower().send_immediate();
        let mut cmd = MpsseCmdBuilder::new();
        let mut pins: Vec<u8> = vec![0];
        let mut ack: Vec<u8> = vec![0];

        // get current state of low pins
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd_read_low_pins.as_slice())?;
        ftdi.read_exact(&mut pins)?;

        // ST: send using bit-banging
        cmd = self.i2c_start(cmd, pins[0]);

        // SAD + R: send using MPSSE
        cmd = self.i2c_write_byte_ack(cmd, I2cBus::i2c_read_from(address), pins[0]);

        // send command and read back one bit
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd.as_slice())?;
        ftdi.read_exact(&mut ack)?;

        // check ACK bit from slave
        if ack[0] & 0x1 == 0x1 {
            return Err(X232Error::HAL(ErrorKind::I2cNoAck));
        }

        // READ bytes from slave
        for i in 0..buffer.len() {
            let mut cmd = MpsseCmdBuilder::new();
            let mut data: Vec<u8> = vec![0, 0];
            let nack: bool = i == (buffer.len() - 1);

            cmd = self.i2c_read_byte(cmd, nack, pins[0]);

            ftdi.usb_purge_buffers()?;
            ftdi.write_all(cmd.as_slice())?;
            ftdi.read_exact(&mut data)?;

            buffer[i] = data[0];
        }

        let mut cmd = MpsseCmdBuilder::new();

        // SP: send using bit-banging
        cmd = self.i2c_stop(cmd, pins[0]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd.as_slice())?;

        Ok(())
    }
}

impl<'a> embedded_hal::blocking::i2c::Write for I2cBus<'a> {
    type Error = X232Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<()> {
        if bytes.is_empty() {
            return Ok(());
        }

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        let cmd_read_low_pins = MpsseCmdBuilder::new().gpio_lower().send_immediate();
        let mut cmd = MpsseCmdBuilder::new();
        let mut pins: Vec<u8> = vec![0];
        let mut ack: Vec<u8> = vec![0];

        // get current state of low pins
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd_read_low_pins.as_slice())?;
        ftdi.read_exact(&mut pins)?;

        // ST: send using bit-banging
        cmd = self.i2c_start(cmd, pins[0]);

        // SAD + W: send using MPSSE
        cmd = self.i2c_write_byte_ack(cmd, I2cBus::i2c_write_to(address), pins[0]);

        // send command and read back one bit
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd.as_slice())?;
        ftdi.read_exact(&mut ack)?;

        // check ACK bit from slave
        if ack[0] & 0x1 == 0x1 {
            return Err(X232Error::HAL(ErrorKind::I2cNoAck));
        }

        // WRITE bytes to slave
        for byte in bytes {
            let mut cmd = MpsseCmdBuilder::new();

            cmd = self.i2c_write_byte_ack(cmd, *byte, pins[0]);

            // send command and read back one bit
            ftdi.usb_purge_buffers()?;
            ftdi.write_all(cmd.as_slice())?;
            ftdi.read_exact(&mut ack)?;

            // check ACK bit from slave
            if ack[0] & 0x1 == 0x1 {
                return Err(X232Error::HAL(ErrorKind::I2cNoAck));
            }
        }

        let mut cmd = MpsseCmdBuilder::new();

        // SP: send using bit-banging
        cmd = self.i2c_stop(cmd, pins[0]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd.as_slice())?;

        Ok(())
    }
}

impl<'a> embedded_hal::blocking::i2c::WriteRead for I2cBus<'a> {
    type Error = X232Error;

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<()> {
        // FIXME: simplified: do not fallback to Read or Write, just throw error
        if bytes.is_empty() || buffer.is_empty() {
            return Err(X232Error::HAL(ErrorKind::InvalidParams));
        }

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        let cmd_read_low_pins = MpsseCmdBuilder::new().gpio_lower().send_immediate();
        let mut cmd = MpsseCmdBuilder::new();
        let mut pins: Vec<u8> = vec![0];
        let mut ack: Vec<u8> = vec![0];

        // get current state of low pins
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd_read_low_pins.as_slice())?;
        ftdi.read_exact(&mut pins)?;

        // ST: send using bit-banging
        cmd = self.i2c_start(cmd, pins[0]);

        // SAD + W: send using MPSSE
        cmd = self.i2c_write_byte_ack(cmd, I2cBus::i2c_write_to(address), pins[0]);

        // send command and read back one bit
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd.as_slice())?;
        ftdi.read_exact(&mut ack)?;

        // check ACK bit from slave
        if ack[0] & 0x1 == 0x1 {
            return Err(X232Error::HAL(ErrorKind::I2cNoAck));
        }

        // WRITE bytes to slave
        for byte in bytes {
            let mut cmd = MpsseCmdBuilder::new();

            cmd = self.i2c_write_byte_ack(cmd, *byte, pins[0]);

            // send command and read back one bit
            ftdi.usb_purge_buffers()?;
            ftdi.write_all(cmd.as_slice())?;
            ftdi.read_exact(&mut ack)?;

            // check ACK bit from slave
            if ack[0] & 0x1 == 0x1 {
                return Err(X232Error::HAL(ErrorKind::I2cNoAck));
            }
        }

        let mut cmd = MpsseCmdBuilder::new();
        let mut ack: Vec<u8> = vec![0];

        // SR: send using bit-banging
        cmd = self.i2c_start(cmd, pins[0]);

        // SAD + R: send using MPSSE
        cmd = self.i2c_write_byte_ack(cmd, I2cBus::i2c_read_from(address), pins[0]);

        // send command and read back one bit
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd.as_slice())?;
        ftdi.read_exact(&mut ack)?;

        // check ACK bit from slave
        if ack[0] & 0x1 == 0x1 {
            return Err(X232Error::HAL(ErrorKind::I2cNoAck));
        }

        // READ bytes from slave
        for i in 0..buffer.len() {
            let mut cmd = MpsseCmdBuilder::new();
            let mut data: Vec<u8> = vec![0, 0];
            let nack: bool = i == (buffer.len() - 1);

            cmd = self.i2c_read_byte(cmd, nack, pins[0]);

            ftdi.usb_purge_buffers()?;
            ftdi.write_all(cmd.as_slice())?;
            ftdi.read_exact(&mut data)?;

            buffer[i] = data[0];
        }

        let mut cmd = MpsseCmdBuilder::new();

        // SP: send using bit-banging
        cmd = self.i2c_stop(cmd, pins[0]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(cmd.as_slice())?;

        Ok(())
    }
}
