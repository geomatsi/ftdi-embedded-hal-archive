use ftdi::BitMode;
use ftdi::FlowControl;

use crate::mpsse::MPSSECmd;
use crate::mpsse::MPSSECmd_H;

use hal::spi::{MODE_0, Mode};
use std::io::{Result, Write};

pub struct FtdiDevice {
    mode: Mode,
    loopback: bool,
    pub ctx: ftdi::Context,
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

        // init gpio to high
        context.write_all(&vec![MPSSECmd::SET_BITS_LOW.into(), 0x0, 0xfb])?;
        context.write_all(&vec![MPSSECmd::SET_BITS_HIGH.into(), 0x0, 0xff])?;

        // set speed
        context.write_all(&vec![MPSSECmd_H::EN_DIV_5.into()])?;
        context.write_all(&vec![MPSSECmd::TCK_DIVISOR.into(), 59, 0])?;

        let m = match mode {
            None => MODE_0,
            Some(mode) => mode,
        };

        let d = FtdiDevice {
            mode: m,
            ctx: context,
            loopback: false,
        };

        Ok(d)
    }

    pub fn spi_mode(&mut self, mode: Mode) {
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
}
