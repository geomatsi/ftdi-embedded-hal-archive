use crate::mpsse::MPSSECmd;

use std::cell::RefCell;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

pub struct I2cBus<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Context>>,
    speed: u32,
}

impl<'a> I2cBus<'a> {
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Context>>) -> I2cBus {
        I2cBus { ctx, speed: 0 }
    }

    pub fn set_speed(mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn get_speed(self) -> u32 {
        self.speed
    }

}

impl<'a> I2cBus<'a> {
    fn i2c_start(&self, pins: u8) -> Vec<u8> {
        let delay = Duration::from_micros(5);
        let mut cmd: Vec<u8> = vec![];

        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins & 0b1111_1000) | 0b11,
            0b1111_1011,
        ]);

        sleep(delay);

        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins & 0b1111_1000) | 0b01,
            0b1111_1011,
        ]);

        sleep(delay);

        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins & 0b1111_1000) | 0b00,
            0b1111_1011,
        ]);

        cmd
    }

    fn i2c_stop(&self, pins: u8) -> Vec<u8> {
        let delay = Duration::from_micros(5);
        let mut cmd: Vec<u8> = vec![];

        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins & 0b1111_1000) | 0b01,
            0b1111_1011,
        ]);

        sleep(delay);

        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins & 0b1111_1000) | 0b11,
            0b1111_1011,
        ]);

        sleep(delay);

        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins & 0b1111_1100) | 0b00,
            0b1111_1000,
        ]);

        cmd
    }
}

impl<'a> embedded_hal::blocking::i2c::Read for I2cBus<'a> {
    type Error = Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<()> {
        if buffer.is_empty() {
            return Ok(());
        }

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        let mut pins: Vec<u8> = vec![0];
        let mut ack: Vec<u8> = vec![0];

        // get current state of low pins

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&[MPSSECmd::GET_BITS_LOW.into(), MPSSECmd::SEND_BACK_NOW.into()]).unwrap();
        ftdi.read_exact(&mut pins).unwrap();

        //
        // ST: send using bit-banging
        //

        let mut cmd: Vec<u8> = self.i2c_start(pins[0]);

        //
        // SAD + R: send using MPSSE
        //

        // send addr and R(1) bit
        cmd.append(&mut vec![
            MPSSECmd::MSB_BYTES_W_FALLING.into(),
            0x0,
            0x0,
            (address << 1) | 0x1,
        ]);

        // DO/DI input, SK output/down
        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins[0] & 0b1111_1000) | 0b00,
            0b1111_1001,
        ]);

        // SAK: recv using bit MPSSE
        cmd.append(&mut vec![MPSSECmd::MSB_BITS_R_RISING.into(), 0x0]);

        // request immediate response from FTDI to host
        cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

        // send command and read back one bit
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;
        ftdi.read_exact(&mut ack)?;

        // check ACK bit from slave
        if ack[0] & 0x1 == 0x1 {
            return Err(Error::new(ErrorKind::Other, "No ACK from slave"))
        }

        //
        // READ bytes from slave
        //

        for i in 0..buffer.len() {
            let mut cmd: Vec<u8> = vec![];
            let mut data: Vec<u8> = vec![0, 0];

            // make sure no occasional SP: SDA output(1) SCL output(0)
            cmd.append(&mut vec![
                MPSSECmd::SET_BITS_LOW.into(),
                (pins[0] & 0b1111_1000) | 0b10,
                0b1111_1011,
            ]);

            // prepare to read: SDA input, SCL output(0)
            cmd.append(&mut vec![
                MPSSECmd::SET_BITS_LOW.into(),
                (pins[0] & 0b1111_1000) | 0b000,
                0b1111_1001,
            ]);

            // read byte using MPSSE
            cmd.append(&mut vec![MPSSECmd::MSB_BYTES_R_FALLING.into(), 0x0, 0x0]);

            if i == (buffer.len() - 1) {
                // last byte => NACK to slave: SDA output(1)
                cmd.append(&mut vec![
                           MPSSECmd::SET_BITS_LOW.into(),
                           (pins[0] & 0b1111_1000) | 0b10,
                           0b1111_1011,
                ]);
            }

            // NACK/ACK to slave: line to zero, so we pretend we read it
            cmd.append(&mut vec![MPSSECmd::MSB_BITS_R_RISING.into(), 0x0]);

            // request immediate response from FTDI to PC
            cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

            ftdi.usb_purge_buffers()?;
            ftdi.write_all(&cmd)?;
            ftdi.read_exact(&mut data)?;

            buffer[i] = data[0];
        }

        //
        // SP: send using bit-banging
        //

        let cmd: Vec<u8> = self.i2c_stop(pins[0]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;

        Ok(())
    }
}

impl<'a> embedded_hal::blocking::i2c::Write for I2cBus<'a> {
    type Error = Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<()> {
        if bytes.is_empty() {
            return Ok(());
        }

        println!("WRITE {} bytes to addr[{:b}]", bytes.len(), address);

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        let mut pins: Vec<u8> = vec![0];
        let mut ack: Vec<u8> = vec![0];

        // get current state of low pins

        let cmd: Vec<u8> = vec![
            MPSSECmd::GET_BITS_LOW.into(),
            MPSSECmd::SEND_BACK_NOW.into()
        ];

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&cmd)?;
        ftdi.read_exact(&mut pins).unwrap();

        //
        // ST: send using bit-banging
        //

        let mut cmd: Vec<u8> = self.i2c_start(pins[0]);

        //
        // SAD + W: send using MPSSE
        //

        // send addr and R(1) bit
        cmd.append(&mut vec![
            MPSSECmd::MSB_BYTES_W_FALLING.into(),
            0x0,
            0x0,
            (address << 1) | 0x0,
        ]);

        // DO/DI input, SK output/down
        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins[0] & 0b1111_1000) | 0b00,
            0b1111_1001,
        ]);

        // SAK: recv using bit MPSSE
        cmd.append(&mut vec![MPSSECmd::MSB_BITS_R_RISING.into(), 0x0]);

        // request immediate response from FTDI to host
        cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

        // send command and read back one bit
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;
        ftdi.read_exact(&mut ack)?;

        // check ACK bit from slave
        if ack[0] & 0x1 == 0x1 {
            return Err(Error::new(ErrorKind::Other, "No ACK from slave"))
        }

        //
        // WRITE bytes to slave
        //

        for i in 0..bytes.len() {
            let mut cmd: Vec<u8> = vec![];

            // make sure no occasional SP: SDA output(1) SCL output(0)
            cmd.append(&mut vec![
                       MPSSECmd::SET_BITS_LOW.into(),
                       (pins[0] & 0b1111_1000) | 0b10,
                       0b1111_1011,
            ]);

            // send addr and R(1) bit
            cmd.append(&mut vec![
                       MPSSECmd::MSB_BYTES_W_FALLING.into(),
                       0x0,
                       0x0,
                       bytes[i]
            ]);

            // DO/DI input, SK output/down
            cmd.append(&mut vec![
                       MPSSECmd::SET_BITS_LOW.into(),
                       (pins[0] & 0b1111_1000) | 0b00,
                       0b1111_1001,
            ]);

            // SAK: recv using bit MPSSE
            cmd.append(&mut vec![MPSSECmd::MSB_BITS_R_RISING.into(), 0x0]);

            // request immediate response from FTDI to host
            cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

            // send command and read back one bit
            ftdi.usb_purge_buffers()?;
            ftdi.write_all(&cmd)?;
            ftdi.read_exact(&mut ack)?;

            // check ACK bit from slave
            if ack[0] & 0x1 == 0x1 {
                return Err(Error::new(ErrorKind::Other, "No ACK from slave"))
            }

        }

        //
        // SP: send using bit-banging
        //

        let cmd: Vec<u8> = self.i2c_stop(pins[0]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;

        Ok(())
    }
}

impl<'a> embedded_hal::blocking::i2c::WriteRead for I2cBus<'a> {
    type Error = Error;

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<()> {
        // FIXME: simplified: do not fallback to Read or Write, just throw error
        if bytes.is_empty() || buffer.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Empty input or output buffer"))
        }

        println!("WRITE_READ[{:b}]: write {} bytes read {} bytes", address, bytes.len(), buffer.len());

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        let mut pins: Vec<u8> = vec![0];
        let mut ack: Vec<u8> = vec![0];

        // get current state of low pins

        let cmd: Vec<u8> = vec![
            MPSSECmd::GET_BITS_LOW.into(),
            MPSSECmd::SEND_BACK_NOW.into()
        ];

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&cmd).unwrap();
        ftdi.read_exact(&mut pins).unwrap();

        //
        // ST: send using bit-banging
        //

        let mut cmd: Vec<u8> = self.i2c_start(pins[0]);

        //
        // SAD + W: send using MPSSE
        //

        // send addr and R(1) bit
        cmd.append(&mut vec![
            MPSSECmd::MSB_BYTES_W_FALLING.into(),
            0x0,
            0x0,
            (address << 1) | 0x0,
        ]);

        // DO/DI input, SK output/down
        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins[0] & 0b1111_1000) | 0b00,
            0b1111_1001,
        ]);

        // SAK: recv using bit MPSSE
        cmd.append(&mut vec![MPSSECmd::MSB_BITS_R_RISING.into(), 0x0]);

        // request immediate response from FTDI to host
        cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

        // send command and read back one bit
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;
        ftdi.read_exact(&mut ack)?;

        // check ACK bit from slave
        if ack[0] & 0x1 == 0x1 {
            return Err(Error::new(ErrorKind::Other, "No ACK from slave"))
        }

        //
        // WRITE bytes to slave
        //

        for i in 0..bytes.len() {
            let mut cmd: Vec<u8> = vec![];

            // make sure no occasional SP: SDA output(1) SCL output(0)
            cmd.append(&mut vec![
                       MPSSECmd::SET_BITS_LOW.into(),
                       (pins[0] & 0b1111_1000) | 0b10,
                       0b1111_1011,
            ]);

            // send addr and R(1) bit
            cmd.append(&mut vec![
                       MPSSECmd::MSB_BYTES_W_FALLING.into(),
                       0x0,
                       0x0,
                       bytes[i]
            ]);

            // DO/DI input, SK output/down
            cmd.append(&mut vec![
                       MPSSECmd::SET_BITS_LOW.into(),
                       (pins[0] & 0b1111_1000) | 0b00,
                       0b1111_1001,
            ]);

            // SAK: recv using bit MPSSE
            cmd.append(&mut vec![MPSSECmd::MSB_BITS_R_RISING.into(), 0x0]);

            // request immediate response from FTDI to host
            cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

            // send command and read back one bit
            ftdi.usb_purge_buffers()?;
            ftdi.write_all(&cmd)?;
            ftdi.read_exact(&mut ack)?;

            // check ACK bit from slave
            if ack[0] & 0x1 == 0x1 {
                return Err(Error::new(ErrorKind::Other, "No ACK from slave"))
            }

        }

        let mut ack: Vec<u8> = vec![0];
        let mut cmd: Vec<u8> = vec![];

        // make sure no occasional SP: SDA output(1) SCL output(0)
        cmd.append(&mut vec![
                   MPSSECmd::SET_BITS_LOW.into(),
                   (pins[0] & 0b1111_1000) | 0b10,
                   0b1111_1011,
        ]);

        //
        // SR: send using bit-banging
        //

        cmd.append(&mut self.i2c_start(pins[0]));

        //
        // SAD + R: send using MPSSE
        //

        // send addr and R(1) bit
        cmd.append(&mut vec![
            MPSSECmd::MSB_BYTES_W_FALLING.into(),
            0x0,
            0x0,
            (address << 1) | 0x1,
        ]);

        // DO/DI input, SK output/down
        cmd.append(&mut vec![
            MPSSECmd::SET_BITS_LOW.into(),
            (pins[0] & 0b1111_1000) | 0b00,
            0b1111_1001,
        ]);

        // SAK: recv using bit MPSSE
        cmd.append(&mut vec![MPSSECmd::MSB_BITS_R_RISING.into(), 0x0]);

        // request immediate response from FTDI to host
        cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

        // send command and read back one bit
        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;
        ftdi.read_exact(&mut ack)?;

        // check ACK bit from slave
        if ack[0] & 0x1 == 0x1 {
            return Err(Error::new(ErrorKind::Other, "No ACK from slave"))
        }

        //
        // READ bytes from slave
        //

        for i in 0..buffer.len() {
            let mut cmd: Vec<u8> = vec![];
            let mut data: Vec<u8> = vec![0, 0];

            // make sure no occasional SP: SDA output(1) SCL output(0)
            cmd.append(&mut vec![
                MPSSECmd::SET_BITS_LOW.into(),
                (pins[0] & 0b1111_1000) | 0b10,
                0b1111_1011,
            ]);

            // prepare to read: SDA input, SCL output(0)
            cmd.append(&mut vec![
                MPSSECmd::SET_BITS_LOW.into(),
                (pins[0] & 0b1111_1000) | 0b000,
                0b1111_1001,
            ]);

            // read byte using MPSSE
            cmd.append(&mut vec![MPSSECmd::MSB_BYTES_R_FALLING.into(), 0x0, 0x0]);

            if i == (buffer.len() - 1) {
                // last byte => NACK to slave: SDA output(1)
                cmd.append(&mut vec![
                           MPSSECmd::SET_BITS_LOW.into(),
                           (pins[0] & 0b1111_1000) | 0b10,
                           0b1111_1011,
                ]);
            } else {
                // not last byte => ACK to slave: SDA output(1)
                cmd.append(&mut vec![
                           MPSSECmd::SET_BITS_LOW.into(),
                           (pins[0] & 0b1111_1000) | 0b00,
                           0b1111_1011,
                ]);
            }

            // NACK/ACK to slave: line to zero, so we pretend we read it
            cmd.append(&mut vec![MPSSECmd::MSB_BITS_R_RISING.into(), 0x0]);

            // request immediate response from FTDI to PC
            cmd.append(&mut vec![MPSSECmd::SEND_BACK_NOW.into()]);

            ftdi.usb_purge_buffers()?;
            ftdi.write_all(&cmd)?;
            ftdi.read_exact(&mut data)?;

            buffer[i] = data[0];
        }

        //
        // SP: send using bit-banging
        //

        let cmd: Vec<u8> = self.i2c_stop(pins[0]);

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(&cmd)?;

        Ok(())
    }
}
