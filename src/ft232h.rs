use ftdi::BitMode;
use ftdi::FlowControl;

use crate::mpsse::MPSSECmd;
use crate::mpsse::MPSSECmd_H;

use hal::spi::{MODE_0, Mode};
use std::io::{Result, Write};

pub enum FtdiPinType {
    Low,
    High,
}

pub enum FtdiPin {
    PinL0,
    PinL1,
    PinL2,
    PinL3,

    PinH0,
    PinH1,
    PinH2,
    PinH3,
    PinH4,
    PinH5,
    PinH6,
    PinH7,
}

impl FtdiPin {
    pub fn bit(&mut self) -> u8 {
        let bit = match self {
            FtdiPin::PinL0 => 0b0001_0000,
            FtdiPin::PinL1 => 0b0010_0000,
            FtdiPin::PinL2 => 0b0100_0000,
            FtdiPin::PinL3 => 0b1000_0000,

            FtdiPin::PinH0 => 0b0000_0001,
            FtdiPin::PinH1 => 0b0000_0010,
            FtdiPin::PinH2 => 0b0000_0100,
            FtdiPin::PinH3 => 0b0000_1000,
            FtdiPin::PinH4 => 0b0001_0000,
            FtdiPin::PinH5 => 0b0010_0000,
            FtdiPin::PinH6 => 0b0100_0000,
            FtdiPin::PinH7 => 0b1000_0000,
        };

        bit as u8
    }

    pub fn bank(&mut self) -> FtdiPinType {
        let bank = match self {
            FtdiPin::PinL0 => FtdiPinType::Low,
            FtdiPin::PinL1 => FtdiPinType::Low,
            FtdiPin::PinL2 => FtdiPinType::Low,
            FtdiPin::PinL3 => FtdiPinType::Low,

            FtdiPin::PinH0 => FtdiPinType::High,
            FtdiPin::PinH1 => FtdiPinType::High,
            FtdiPin::PinH2 => FtdiPinType::High,
            FtdiPin::PinH3 => FtdiPinType::High,
            FtdiPin::PinH4 => FtdiPinType::High,
            FtdiPin::PinH5 => FtdiPinType::High,
            FtdiPin::PinH6 => FtdiPinType::High,
            FtdiPin::PinH7 => FtdiPinType::High,
        };

        bank as FtdiPinType
    }
}

pub struct FtdiDevice {
    pub ctx: ftdi::Context,
    pub pin: FtdiPin,

    loopback: bool,
    mode: Mode,
}

impl FtdiDevice {
    pub fn spi_init(vendor: u16, product: u16, mode: Option<Mode>) -> Result<FtdiDevice> {
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

        // default gpio configuration
        FtdiDevice::gpio_init(&mut context)?;

        let m = match mode {
            None => MODE_0,
            Some(mode) => mode,
        };

        let d = FtdiDevice {
            ctx: context,
            pin: FtdiPin::PinL0,
            loopback: false,
            mode: m,
        };

        Ok(d)
    }

    pub fn select_pin(&mut self, pin: FtdiPin) -> &mut FtdiDevice {
        self.pin = pin;
        self
    }

    pub fn spi_mode(mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn loopback(&mut self, lp: bool) -> Result<()> {
        self.loopback = lp;

        let cmd = match lp {
            true => MPSSECmd::LOOPBACK_START,
            false => MPSSECmd::LOOPBACK_END,
        };

        self.ctx.write_all(&vec![cmd.into()])?;
        Ok(())
    }

    fn gpio_init(ctx: &mut ftdi::Context) -> Result<()> {
        // FIXME: current approach is limited: fixed in/out pin configuration:

        // low bits: DI (0b0100) input, other outputs
        // all outputs initially zeros
        ctx.write_all(&vec![MPSSECmd::SET_BITS_LOW.into(), 0x0, 0b1111_1011])?;

        // high bits: all outputs
        // all outputs initially zeros
        ctx.write_all(&vec![MPSSECmd::SET_BITS_HIGH.into(), 0x0, 0b1111_1111])?;
        Ok(())
    }
}
