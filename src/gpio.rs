use crate::error::{Result, X232Error};
use crate::ftdimpsse::MpsseCmdBuilder;

use embedded_hal::digital::v2::OutputPin;
use std::cell::RefCell;
use std::fmt;
use std::io::{Read, Write};
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinBank {
    Low,
    High,
}

impl fmt::Display for PinBank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PinBank::Low => write!(f, "L"),
            PinBank::High => write!(f, "H"),
        }
    }
}

#[macro_export]
macro_rules! declare_gpio_pin {
    ($pin: ident, $bit: expr, $bank: expr) => {
        pub fn $pin(&self) -> Result<GpioPin> {
            if !*self.$pin.borrow() {
                return Err(X232Error::HAL(ErrorKind::GpioPinBusy));
            }

            if $bit > 7 {
                return Err(X232Error::HAL(ErrorKind::GpioInvalidPin));
            }

            self.$pin.replace(false);
            Ok(GpioPin::new(&self.mtx, $bit, $bank))
        }
    };
}

pub struct GpioPin<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Device>>,
    bank: PinBank,
    bit: u8,
}

impl<'a> fmt::Display for GpioPin<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.bank {
            PinBank::Low => write!(f, "P{}{}", self.bank, self.bit - 4),
            PinBank::High => write!(f, "P{}{}", self.bank, self.bit),
        }
    }
}

impl<'a> GpioPin<'a> {
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Device>>, bit: u8, bank: PinBank) -> GpioPin {
        GpioPin { ctx, bank, bit }
    }

    pub fn get_bit(&self) -> u8 {
        self.bit
    }

    pub fn get_bank(&self) -> PinBank {
        self.bank
    }

    fn set_pin(&mut self, val: bool) -> Result<()> {
        let mut value: [u8; 1] = [0];

        let read = match self.bank {
            PinBank::Low => MpsseCmdBuilder::new().gpio_lower().send_immediate(),
            PinBank::High => MpsseCmdBuilder::new().gpio_upper().send_immediate(),
        };

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(read.as_slice())?;
        ftdi.read_exact(&mut value)?;

        let v = if val {
            value[0] | (1 << self.bit)
        } else {
            value[0] & (!(1 << self.bit))
        };

        let write = match self.bank {
            PinBank::Low => MpsseCmdBuilder::new()
                .set_gpio_lower(v, 0b1111_1011)
                .send_immediate(),
            PinBank::High => MpsseCmdBuilder::new()
                .set_gpio_upper(v, 0b1111_1111)
                .send_immediate(),
        };

        ftdi.usb_purge_buffers()?;
        ftdi.write_all(write.as_slice())?;

        Ok(())
    }
}

impl<'a> OutputPin for GpioPin<'a> {
    type Error = X232Error;

    fn set_low(&mut self) -> Result<()> {
        self.set_pin(false)?;
        Ok(())
    }

    fn set_high(&mut self) -> Result<()> {
        self.set_pin(true)?;
        Ok(())
    }
}
