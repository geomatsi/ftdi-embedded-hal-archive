use ftdi::BitMode;
pub use ftdi::Interface;

use crate::mpsse::MPSSECmd;
use crate::mpsse::MPSSECmd_H;

use crate::gpio::GpioPin;
use crate::gpio::PinBank;
use crate::i2c::I2cBus;
use crate::i2c::I2cSpeed;
use crate::spi::SpiBus;
use crate::spi::SpiSpeed;

use std::cell::RefCell;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::io::Write;
use std::sync::Mutex;

pub struct FTx232H {
    mtx: Mutex<RefCell<ftdi::Context>>,
    loopback: bool,

    i2c: RefCell<Option<I2cSpeed>>,
    spi: RefCell<Option<SpiSpeed>>,

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

impl FTx232H {
    pub fn init(vendor: u16, product: u16) -> Result<FTx232H> {
        FTx232H::init_ctx(vendor, product, ftdi::Interface::A)
    }

    pub fn init_chan(vendor: u16, product: u16, intf: ftdi::Interface) -> Result<FTx232H> {
        FTx232H::init_ctx(vendor, product, intf)
    }

    fn init_ctx(vendor: u16, product: u16, intf: ftdi::Interface) -> Result<FTx232H> {
        let mut context = ftdi::Context::new();

        context.set_interface(intf)?;

        if context.usb_open(vendor, product).is_err() {
            panic!("No FTDI device");
        }

        context.set_write_chunksize(1024);
        context.set_read_chunksize(1024);
        context.usb_reset()?;
        context.set_latency_timer(5)?;
        context.set_bitmode(0, BitMode::MPSSE)?;
        context.usb_purge_buffers()?;

        // clock settings:
        // - disable DIV_5 => 60MHz
        // - disable adaptive clocking
        // - disable 3-phase clocking
        context.write_all(&[MPSSECmd_H::DISABLE_DIV_5_CLK.into()])?;
        context.write_all(&[MPSSECmd_H::DISABLE_ADAPTIVE_CLK.into()])?;
        context.write_all(&[MPSSECmd_H::DISABLE_3_PHASE_CLK.into()])?;

        // disable loopback
        context.write_all(&[MPSSECmd::LOOPBACK_DISABLE.into()])?;

        // FIXME: current approach is limited: fixed in/out pin configuration:
        // - low bits: all outputs(0)
        context.write_all(&[MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1111])?;

        // FIXME: current approach is limited: fixed in/out pin configuration:
        // - high bits: all outputs(0)
        context.write_all(&[MPSSECmd::SET_BITS_HIGH.into(), 0x0, 0b1111_1111])?;

        let d = FTx232H {
            mtx: Mutex::new(RefCell::new(context)),
            loopback: false,

            i2c: RefCell::new(None),
            spi: RefCell::new(None),

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
            MPSSECmd::LOOPBACK_ENABLE
        } else {
            MPSSECmd::LOOPBACK_DISABLE
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

    pub fn spi(&self, speed: SpiSpeed) -> Result<SpiBus> {
        if (*self.i2c.borrow()).is_some() {
            return Err(Error::new(ErrorKind::Other, "can't use spi: i2c is active"));
        }

        if (*self.spi.borrow()).is_none() {
            let lock = self.mtx.lock().unwrap();
            let mut ftdi = lock.borrow_mut();

            self.spi.replace(Some(speed));

            // SPI: DI - input, DO - output(0), SK - output(0)
            ftdi.write_all(&[MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1011])?;

            // FIXME: set fixed speed 1MHz for all devices assuming 60MHz clock
            // SCK_freq = 60MHz / ((1 + (div1 | (div2 << 8))) * 2)
            let (div1, div2) = match speed {
                SpiSpeed::CLK_500kHz => (0x3b, 0x0),
                SpiSpeed::CLK_1MHz | SpiSpeed::CLK_AUTO => (0x1d, 0x0),
                SpiSpeed::CLK_3MHz => (0x9, 0x0),
                SpiSpeed::CLK_5MHz => (0x5, 0x0),
            };

            ftdi.write_all(&[MPSSECmd::TCK_DIVISOR.into(), div1, div2])?;
        } else if speed != SpiSpeed::CLK_AUTO {
            // clock sanity check
            if Some(speed) != *self.spi.borrow() {
                return Err(Error::new(ErrorKind::Other, "spi clock mismatch"));
            }
        }

        Ok(SpiBus::new(&self.mtx))
    }

    pub fn i2c(&self, speed: I2cSpeed) -> Result<I2cBus> {
        if (*self.spi.borrow()).is_some() {
            return Err(Error::new(ErrorKind::Other, "can't use i2c: spi is active"));
        }

        if (*self.i2c.borrow()).is_none() {
            let lock = self.mtx.lock().unwrap();
            let mut ftdi = lock.borrow_mut();

            self.i2c.replace(Some(speed));

            // I2C: DI - input, DO - output(0), SK - output(0)
            ftdi.write_all(&[MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1011])?;

            // FIXME: set fixed speed 1MHz for all devices assuming 60MHz clock
            // SCK_freq = 60MHz / ((1 + (div1 | (div2 << 8))) * 2)
            let (div1, div2) = match speed {
                I2cSpeed::CLK_100kHz | I2cSpeed::CLK_AUTO => (0x2b, 0x1),
                I2cSpeed::CLK_400kHz => (0x4a, 0x0),
            };

            ftdi.write_all(&[MPSSECmd::TCK_DIVISOR.into(), div1, div2])?;
        } else if speed != I2cSpeed::CLK_AUTO {
            // clock sanity check
            if Some(speed) != *self.i2c.borrow() {
                return Err(Error::new(ErrorKind::Other, "i2c clock mismatch"));
            }
        }

        Ok(I2cBus::new(&self.mtx))
    }

    // gpio pins: low bank
    crate::declare_gpio_pin!(pl0, 4, PinBank::Low);
    crate::declare_gpio_pin!(pl1, 5, PinBank::Low);
    crate::declare_gpio_pin!(pl2, 6, PinBank::Low);
    crate::declare_gpio_pin!(pl3, 7, PinBank::Low);

    // gpio pins: high bank
    crate::declare_gpio_pin!(ph0, 0, PinBank::High);
    crate::declare_gpio_pin!(ph1, 1, PinBank::High);
    crate::declare_gpio_pin!(ph2, 2, PinBank::High);
    crate::declare_gpio_pin!(ph3, 3, PinBank::High);
    crate::declare_gpio_pin!(ph4, 4, PinBank::High);
    crate::declare_gpio_pin!(ph5, 5, PinBank::High);
    crate::declare_gpio_pin!(ph6, 6, PinBank::High);
    crate::declare_gpio_pin!(ph7, 7, PinBank::High);
}

impl Drop for FTx232H {
    fn drop(&mut self) {
        let lock = self.mtx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.usb_purge_buffers().unwrap();
        ftdi.usb_close().unwrap();
    }
}
