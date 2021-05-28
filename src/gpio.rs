use crate::mpsse::MPSSECmd;

use embedded_hal::digital::v2::OutputPin;
use std::cell::RefCell;
use std::convert::Infallible;
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
                return Err(Error::new(ErrorKind::Other, "pin already in use"));
            }

            if $bit > 7 {
                return Err(Error::new(ErrorKind::Other, "invalid pin number"));
            }

            self.$pin.replace(false);
            Ok(GpioPin::new(&self.mtx, $bit, $bank))
        }
    };
}

pub struct GpioPin<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Context>>,
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
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Context>>, bit: u8, bank: PinBank) -> GpioPin {
        GpioPin { ctx, bank, bit }
    }

    pub fn get_bit(&self) -> u8 {
        self.bit
    }

    pub fn get_bank(&self) -> PinBank {
        self.bank
    }

    fn set_pin(&mut self, val: bool) {
        let (get_cmd, set_cmd, dir) = match self.bank {
            PinBank::Low => (MPSSECmd::GET_BITS_LOW, MPSSECmd::SET_BITS_LOW, 0b1111_1011),
            PinBank::High => (
                MPSSECmd::GET_BITS_HIGH,
                MPSSECmd::SET_BITS_HIGH,
                0b1111_1111,
            ),
        };

        let mut value: Vec<u8> = vec![0];

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&[get_cmd.into()]).unwrap();
        ftdi.read_exact(&mut value).unwrap();

        let v = if val {
            value[0] | (1 << self.bit)
        } else {
            value[0] & (!(1 << self.bit))
        };

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&[set_cmd.into(), v, dir]).unwrap();
    }
}

impl<'a> OutputPin for GpioPin<'a> {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_pin(false);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_pin(true);
        Ok(())
    }
}
