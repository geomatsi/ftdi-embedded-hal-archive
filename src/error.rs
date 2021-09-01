use ftdi;
use libftd2xx;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, X232Error>;

#[derive(Debug)]
pub enum X232Error {
    Io(io::Error),
    FTDI(ftdi::Error),
    HAL(ErrorKind),
    FTD2XX(libftd2xx::TimeoutError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    InvalidParams,
    InvalidClock,
    BusBusy,
    I2cNoAck,
    GpioPinBusy,
    GpioInvalidPin,
    SpiModeNotSupported,
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ErrorKind::InvalidParams => "Invalid input params",
            ErrorKind::BusBusy => "Bus is busy",
            ErrorKind::InvalidClock => "Clock is not valid",
            ErrorKind::I2cNoAck => "No ACK from slave",
            ErrorKind::GpioPinBusy => "GPIO pin is already in use",
            ErrorKind::GpioInvalidPin => "No such GPIO pin",
            ErrorKind::SpiModeNotSupported => "Mode not supported",
        }
    }
}

impl fmt::Display for X232Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            X232Error::Io(ref err) => err.fmt(f),
            X232Error::FTDI(ref err) => err.fmt(f),
            X232Error::FTD2XX(ref err) => err.fmt(f),
            X232Error::HAL(ref err) => write!(f, "A regular error occurred {:?}", err.as_str()),
        }
    }
}

impl From<io::Error> for X232Error {
    fn from(e: io::Error) -> Self {
        X232Error::Io(e)
    }
}
impl From<ftdi::Error> for X232Error {
    fn from(e: ftdi::Error) -> Self {
        X232Error::FTDI(e)
    }
}

impl From<libftd2xx::TimeoutError> for X232Error {
    fn from(e: libftd2xx::TimeoutError) -> Self {
        X232Error::FTD2XX(e)
    }
}
