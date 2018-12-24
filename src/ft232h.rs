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

    i2c: RefCell<bool>,
    spi: RefCell<bool>,

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

macro_rules! declare_gpio_pin {
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

        // clock settings:
        // - disable DIV_5 => 60MHz
        // - disable adaptive clocking
        // - disable 3-phase clocking
        context.write_all(&[MPSSECmd_H::DIS_DIV_5.into()])?;
        context.write_all(&[MPSSECmd_H::DIS_ADAPTIVE.into()])?;
        context.write_all(&[MPSSECmd_H::DIS_3_PHASE.into()])?;

        // disable loopback
        context.write_all(&[MPSSECmd::LOOPBACK_END.into()])?;

        // FIXME: current approach is limited: fixed in/out pin configuration:
        // - low bits: all outputs(0)
        context.write_all(&[MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1111])?;

        // FIXME: current approach is limited: fixed in/out pin configuration:
        // - high bits: all outputs(0)
        context.write_all(&[MPSSECmd::SET_BITS_HIGH.into(), 0x0, 0b1111_1111])?;

        let d = FT232H {
            mtx: Mutex::new(RefCell::new(context)),
            loopback: false,

            i2c: RefCell::new(true),
            spi: RefCell::new(true),

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
        if !*self.i2c.borrow() {
            return Err(Error::new(ErrorKind::Other, "can't use spi: i2c is active"));
        }

        if *self.spi.borrow() {
            let lock = self.mtx.lock().unwrap();
            let mut ftdi = lock.borrow_mut();

            self.spi.replace(false);

            // SPI: DI - input, DO - output(0), SK - output(0)
            ftdi.write_all(&[MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1011])?;

            // FIXME: set fixed speed 1MHz for all devices assuming 60MHz clock
            // SCK_freq = 60MHz / ((1 + 0x1d) * 2) = 1MHz
            ftdi.write_all(&[MPSSECmd::TCK_DIVISOR.into(), 0x1d, 0])?;
        }

        Ok(SpiBus::new(&self.mtx))
    }

    pub fn i2c(&self) -> Result<I2cBus> {
        if !*self.spi.borrow() {
            return Err(Error::new(ErrorKind::Other, "can't use i2c: spi is active"));
        }

        if *self.i2c.borrow() {
            let lock = self.mtx.lock().unwrap();
            let mut ftdi = lock.borrow_mut();

            self.i2c.replace(false);

            // I2C: DI - input, DO - output(0), SK - output(0)
            ftdi.write_all(&[MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1011])?;

            // FIXME: set fixed speed 100KHz for all i2c devices assuming 60MHz clock
            // SCL_freq = 60MHz / ((1 + 0x12b) * 2) = 100KHz
            ftdi.write_all(&[MPSSECmd::TCK_DIVISOR.into(), 0x2b, 0x01])?;
        }

        Ok(I2cBus::new(&self.mtx))
    }

    // gpio pins: low bank
    declare_gpio_pin!(pl0, 0b0001_0000, PinBank::Low);
    declare_gpio_pin!(pl1, 0b0010_0000, PinBank::Low);
    declare_gpio_pin!(pl2, 0b0100_0000, PinBank::Low);
    declare_gpio_pin!(pl3, 0b1000_0000, PinBank::Low);

    // gpio pins: high bank
    declare_gpio_pin!(ph0, 0b0000_0001, PinBank::High);
    declare_gpio_pin!(ph1, 0b0000_0010, PinBank::High);
    declare_gpio_pin!(ph2, 0b0000_0100, PinBank::High);
    declare_gpio_pin!(ph3, 0b0000_1000, PinBank::High);
    declare_gpio_pin!(ph4, 0b0001_0000, PinBank::High);
    declare_gpio_pin!(ph5, 0b0010_0000, PinBank::High);
    declare_gpio_pin!(ph6, 0b0100_0000, PinBank::High);
    declare_gpio_pin!(ph7, 0b1000_0000, PinBank::High);
}

impl Drop for FT232H {
    fn drop(&mut self) {
        let lock = self.mtx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.usb_purge_buffers().unwrap();
        ftdi.usb_close().unwrap();
    }
}
