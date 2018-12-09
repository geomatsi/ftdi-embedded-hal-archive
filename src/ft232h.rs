use ftdi::BitMode;

use crate::mpsse::MPSSECmd;
use crate::mpsse::MPSSECmd_H;

use crate::gpio::GpioPin;
use crate::gpio::PinBank;
use crate::i2c::I2cBus;
use crate::spi::SpiBus;

use std::cell::RefCell;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::io::Write;
use std::sync::Mutex;

pub struct FT232H {
    mtx: Mutex<RefCell<ftdi::Context>>,
    loopback: bool,

    pl0: RefCell<bool>,
    pl1: RefCell<bool>,
    pl2: RefCell<bool>,
    pl3: RefCell<bool>,

    ph0: RefCell<bool>,
    ph1: RefCell<bool>,
    ph2: RefCell<bool>,
    ph3: RefCell<bool>,
    ph4: RefCell<bool>,
    ph5: RefCell<bool>,
    ph6: RefCell<bool>,
    ph7: RefCell<bool>,
}

macro_rules! DECLARE_GPIO_PIN {
    ($pin:ident, $bit:expr, $bank: expr) => (
        pub fn $pin(&self) -> Result<GpioPin> {
            if !*self.$pin.borrow() {
                return Err(Error::new(ErrorKind::Other, "pin already in use"));
            }

            self.$pin.replace(false);
            Ok(GpioPin::new(&self.mtx, $bit, $bank))
        }
    )
}

impl FT232H {
    pub fn init(vendor: u16, product: u16) -> Result<FT232H> {
        let mut context = ftdi::Context::new();

        if context.usb_open(vendor, product).is_err() {
            panic!("No FTDI device");
        }

        context.set_write_chunksize(1024);
        context.set_read_chunksize(1024);
        context.set_interface(ftdi::Interface::A)?;
        context.usb_reset()?;
        context.set_latency_timer(5)?;
        context.set_bitmode(0, BitMode::MPSSE)?;
        context.usb_purge_buffers()?;

        // disable loopback
        context.write_all(&[MPSSECmd::LOOPBACK_END.into()])?;

        // set speed
        context.write_all(&[MPSSECmd_H::EN_DIV_5.into()])?;
        context.write_all(&[MPSSECmd::TCK_DIVISOR.into(), 59, 0])?;

        // FIXME: current approach is limited: fixed in/out pin configuration:
        // low bits: DI (0b0100) input, other outputs
        // all outputs initially zeros
        context.write_all(&[MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1011])?;
        // high bits: all outputs
        // all outputs initially zeros
        context.write_all(&[MPSSECmd::SET_BITS_HIGH.into(), 0x0, 0b1111_1111])?;

        let d = FT232H {
            mtx: Mutex::new(RefCell::new(context)),
            loopback: false,

            pl0: RefCell::new(true),
            pl1: RefCell::new(true),
            pl2: RefCell::new(true),
            pl3: RefCell::new(true),

            ph0: RefCell::new(true),
            ph1: RefCell::new(true),
            ph2: RefCell::new(true),
            ph3: RefCell::new(true),
            ph4: RefCell::new(true),
            ph5: RefCell::new(true),
            ph6: RefCell::new(true),
            ph7: RefCell::new(true),
        };

        Ok(d)
    }

    pub fn loopback(&mut self, lp: bool) -> Result<()> {
        self.loopback = lp;

        let cmd = if lp {
            MPSSECmd::LOOPBACK_START
        } else {
            MPSSECmd::LOOPBACK_END
        };

        let lock = self.mtx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.write_all(&[cmd.into()])?;

        Ok(())
    }

    pub fn is_loopback(&self) -> bool {
        self.loopback
    }

    // spi/i2c buses

    pub fn spi(&self) -> Result<SpiBus> {
        Ok(SpiBus::new(&self.mtx))
    }

    pub fn i2c(&self) -> Result<I2cBus> {
        Ok(I2cBus::new(&self.mtx))
    }

    // gpio pins: low bank

    DECLARE_GPIO_PIN!(pl0, 0b0001_0000, PinBank::Low);
    DECLARE_GPIO_PIN!(pl1, 0b0010_0000, PinBank::Low);
    DECLARE_GPIO_PIN!(pl2, 0b0100_0000, PinBank::Low);
    DECLARE_GPIO_PIN!(pl3, 0b1000_0000, PinBank::Low);

    // gpio pins: high bank

    DECLARE_GPIO_PIN!(ph0, 0b0000_0001, PinBank::High);
    DECLARE_GPIO_PIN!(ph1, 0b0000_0010, PinBank::High);
    DECLARE_GPIO_PIN!(ph2, 0b0000_0100, PinBank::High);
    DECLARE_GPIO_PIN!(ph3, 0b0000_1000, PinBank::High);
    DECLARE_GPIO_PIN!(ph4, 0b0001_0000, PinBank::High);
    DECLARE_GPIO_PIN!(ph5, 0b0010_0000, PinBank::High);
    DECLARE_GPIO_PIN!(ph6, 0b0100_0000, PinBank::High);
    DECLARE_GPIO_PIN!(ph7, 0b1000_0000, PinBank::High);
}

impl Drop for FT232H {
    fn drop(&mut self) {
        let lock = self.mtx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.usb_purge_buffers().unwrap();
        ftdi.usb_close().unwrap();
    }
}
