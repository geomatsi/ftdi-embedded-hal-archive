extern crate embedded_hal as hal;

use ftdi::BitMode;
use ftdi::FlowControl;

use crate::mpsse::MPSSECmd;
use crate::mpsse::MPSSECmd_H;

use crate::gpio::GpioPin;
use crate::gpio::PinBank;
use crate::spi::SpiBus;

use std::cell::RefCell;
use std::io::{Result, Write};
use std::sync::Mutex;

pub struct FT232H {
    mtx: Mutex<RefCell<ftdi::Context>>,
    loopback: bool,
}

impl FT232H {
    pub fn init(vendor: u16, product: u16) -> Result<FT232H> {
        let mut context = ftdi::Context::new();

        if !context.usb_open(vendor, product).is_ok() {
            panic!("No FTDI device");
        }

        context.set_write_chunksize(32);
        context.set_interface(ftdi::Interface::A)?;
        context.usb_reset()?;
        context.set_latency_timer(2)?;
        context.set_flow_control(FlowControl::SIO_RTS_CTS_HS)?;
        context.set_bitmode(0, BitMode::MPSSE)?;
        context.usb_purge_buffers()?;

        // disable loopback
        context.write_all(&vec![MPSSECmd::LOOPBACK_END.into()])?;

        // set speed
        context.write_all(&vec![MPSSECmd_H::EN_DIV_5.into()])?;
        context.write_all(&vec![MPSSECmd::TCK_DIVISOR.into(), 59, 0])?;

        // FIXME: current approach is limited: fixed in/out pin configuration:
        // low bits: DI (0b0100) input, other outputs
        // all outputs initially zeros
        context.write_all(&vec![MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1011])?;
        // high bits: all outputs
        // all outputs initially zeros
        context.write_all(&vec![MPSSECmd::SET_BITS_HIGH.into(), 0x0, 0b1111_1111])?;

        let d = FT232H {
            mtx: Mutex::new(RefCell::new(context)),
            loopback: false,
        };

        Ok(d)
    }

    pub fn loopback(&mut self, lp: bool) -> Result<()> {
        self.loopback = lp;

        let cmd = match lp {
            true => MPSSECmd::LOOPBACK_START,
            false => MPSSECmd::LOOPBACK_END,
        };

        let lock = self.mtx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.write_all(&vec![cmd.into()])?;

        Ok(())
    }

    pub fn is_loopback(&self) -> bool {
        return self.loopback
    }

    pub fn spi(&self) -> Result<SpiBus> {
        Ok(SpiBus::new(&self.mtx))
    }

    pub fn pl2(&self) -> Result<GpioPin> {
        Ok(GpioPin::new(&self.mtx, 0b0100_0000, PinBank::Low))
    }

    pub fn ph0(&self) -> Result<GpioPin> {
        Ok(GpioPin::new(&self.mtx, 0b0000_0001, PinBank::High))
    }
}

//

//FtdiPin::PinL0 => 0b0001_0000,
//FtdiPin::PinL1 => 0b0010_0000,
//FtdiPin::PinL2 => 0b0100_0000,
//FtdiPin::PinL3 => 0b1000_0000,

//FtdiPin::PinH0 => 0b0000_0001,
//FtdiPin::PinH1 => 0b0000_0010,
//FtdiPin::PinH2 => 0b0000_0100,
//FtdiPin::PinH3 => 0b0000_1000,
//FtdiPin::PinH4 => 0b0001_0000,
//FtdiPin::PinH5 => 0b0010_0000,
//FtdiPin::PinH6 => 0b0100_0000,
//FtdiPin::PinH7 => 0b1000_0000,
