use crate::mpsse::MPSSECmd;

use std::cell::RefCell;
use std::io::{Read, Write};
use std::sync::Mutex;

pub enum PinBank {
    Low,
    High,
}

pub struct GpioPin<'a> {
    ctx: &'a Mutex<RefCell<ftdi::Context>>,
    bank: PinBank,
    bit: u8,
}

impl<'a> GpioPin<'a> {
    pub fn new(ctx: &'a Mutex<RefCell<ftdi::Context>>, bit: u8, bank: PinBank) -> GpioPin {
        GpioPin {
            ctx: ctx,
            bit: bit,
            bank: bank,
        }
    }

    pub fn get_bit(&self) -> u8 {
        self.bit
    }

    fn set_pin(&mut self, val: bool) {
        let (get_cmd, set_cmd) = match self.bank {
            PinBank::Low => (MPSSECmd::GET_BITS_LOW, MPSSECmd::SET_BITS_LOW),
            PinBank::High => (MPSSECmd::GET_BITS_HIGH, MPSSECmd::SET_BITS_HIGH),
        };

        let mut value: Vec<u8> = vec![0];

        let lock = self.ctx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&vec![get_cmd.into()]).unwrap();
        ftdi.read_exact(&mut value).unwrap();

        let v = match val {
            false => value[0] & (!self.bit),
            true => value[0] | self.bit,
        };

        ftdi.usb_purge_buffers().unwrap();
        ftdi.write_all(&vec![set_cmd.into(), v, 0b1111_1011])
            .unwrap();
    }
}

impl<'a> embedded_hal::digital::OutputPin for GpioPin<'a> {
    fn set_low(&mut self) {
        self.set_pin(false);
    }

    fn set_high(&mut self) {
        self.set_pin(true);
    }
}
