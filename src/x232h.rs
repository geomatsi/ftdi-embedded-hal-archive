use crate::error::{ErrorKind, Result, X232Error};

use crate::gpio::GpioPin;
use crate::gpio::PinBank;
use crate::i2c::I2cBus;
use crate::spi::SpiBus;

use ftdi_mpsse::MpsseCmdBuilder;
use ftdi_mpsse::MpsseCmdExecutor;
use ftdi_mpsse::MpsseSettings;

use std::cell::RefCell;
use std::sync::Mutex;

pub struct FTx232H<T>
where
    T: MpsseCmdExecutor,
    X232Error: From<<T as MpsseCmdExecutor>::Error>,
{
    mtx: Mutex<RefCell<T>>,
    loopback: bool,

    i2c: RefCell<Option<bool>>,
    spi: RefCell<Option<bool>>,

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

impl<T> FTx232H<T>
where
    T: MpsseCmdExecutor,
    X232Error: From<<T as MpsseCmdExecutor>::Error>,
{
    pub fn init(device: T, frequency: u32) -> Result<FTx232H<T>> {
        let settings: MpsseSettings = MpsseSettings {
            clock_frequency: Some(frequency),
            ..Default::default()
        };

        FTx232H::init_full(device, settings)
    }

    pub fn init_full(mut device: T, settings: MpsseSettings) -> Result<FTx232H<T>> {
        device.mpsse_init(&settings)?;
        // Device settings:
        // - disable adaptive clocking
        // - disable 3-phase clocking
        // - disable loopback
        // - low bits: all outputs(0)
        // FIXME: current approach is limited: fixed in/out pin configuration:
        let cmd_init = MpsseCmdBuilder::new()
            .disable_adaptive_data_clocking()
            .disable_3phase_data_clocking()
            .disable_loopback()
            .set_gpio_lower(0x0, 0b1111_1111)
            .set_gpio_upper(0x0, 0b1111_1111);

        device.mpsse_send(cmd_init.as_slice())?;

        let d = FTx232H {
            mtx: Mutex::new(RefCell::new(device)),
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
            MpsseCmdBuilder::new().enable_loopback()
        } else {
            MpsseCmdBuilder::new().disable_loopback()
        };

        let lock = self.mtx.lock().unwrap();
        let mut ftdi = lock.borrow_mut();

        ftdi.mpsse_send(cmd.as_slice())?;

        Ok(())
    }

    pub fn is_loopback(&self) -> bool {
        self.loopback
    }

    // spi/i2c buses

    pub fn spi(&self) -> Result<SpiBus<T>> {
        if (*self.i2c.borrow()).is_some() {
            return Err(X232Error::HAL(ErrorKind::BusBusy));
        }

        if (*self.spi.borrow()).is_none() {
            let lock = self.mtx.lock().unwrap();
            let mut ftdi = lock.borrow_mut();

            self.spi.replace(Some(true));

            // SPI: DI - input, DO - output(0), SK - output(0)
            ftdi.mpsse_send(
                MpsseCmdBuilder::new()
                    .set_gpio_lower(0x0, 0b1111_1011)
                    .as_slice(),
            )?;
        }

        Ok(SpiBus::new(&self.mtx))
    }

    pub fn i2c(&self) -> Result<I2cBus<T>> {
        if (*self.spi.borrow()).is_some() {
            return Err(X232Error::HAL(ErrorKind::BusBusy));
        }

        if (*self.i2c.borrow()).is_none() {
            let lock = self.mtx.lock().unwrap();
            let mut ftdi = lock.borrow_mut();

            self.i2c.replace(Some(true));

            // I2C: DI - input, DO - output(0), SK - output(0)
            ftdi.mpsse_send(
                MpsseCmdBuilder::new()
                    .set_gpio_lower(0x0, 0b1111_1011)
                    .as_slice(),
            )?;
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

impl<T> Drop for FTx232H<T>
where
    T: MpsseCmdExecutor,
    X232Error: From<<T as MpsseCmdExecutor>::Error>,
{
    fn drop(&mut self) {
        let lock = match self.mtx.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mut ftdi = lock.borrow_mut();
        ftdi.mpsse_close();
    }
}
