pub use hal::digital::OutputPin;

use crate::ft232h::{FtdiDevice, FtdiPinType};
use crate::mpsse::MPSSECmd;
use std::io::{Read, Write};

impl hal::digital::OutputPin for FtdiDevice {
    fn set_low(&mut self) {
        let (get_cmd, set_cmd) = match self.pin.bank() {
            FtdiPinType::Low => (MPSSECmd::GET_BITS_LOW, MPSSECmd::SET_BITS_LOW),
            FtdiPinType::High => (MPSSECmd::GET_BITS_HIGH, MPSSECmd::SET_BITS_HIGH),
        };

        let mut value: Vec<u8> = vec![0];
        let bit: u8 = self.pin.bit();

        self.ctx.usb_purge_buffers().unwrap();
        self.ctx.write_all(&vec![get_cmd.into()]).unwrap();
        self.ctx.read_exact(&mut value).unwrap();

        self.ctx.usb_purge_buffers().unwrap();
        self.ctx
            .write_all(&vec![set_cmd.into(), value[0] & (!bit), 0b1111_1011])
            .unwrap();
    }

    fn set_high(&mut self) {
        let (get_cmd, set_cmd) = match self.pin.bank() {
            FtdiPinType::Low => (MPSSECmd::GET_BITS_LOW, MPSSECmd::SET_BITS_LOW),
            FtdiPinType::High => (MPSSECmd::GET_BITS_HIGH, MPSSECmd::SET_BITS_HIGH),
        };

        let mut value: Vec<u8> = vec![0];
        let bit: u8 = self.pin.bit();

        self.ctx.usb_purge_buffers().unwrap();
        self.ctx.write_all(&vec![get_cmd.into()]).unwrap();
        self.ctx.read_exact(&mut value).unwrap();

        self.ctx.usb_purge_buffers().unwrap();
        self.ctx
            .write_all(&vec![set_cmd.into(), value[0] | bit, 0b1111_1011])
            .unwrap();
    }
}
