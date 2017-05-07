use std::io;
use std::sync;
use std::error;
use std::convert;
use std::fmt;

use log;

/// Convenience error which combines possible which could occur while initializing logging.
///
/// Fern does not use this error natively, but functions which set up fern and open log files will
/// often need to return both io::Error and SetLoggerError. This error is for that purpose.
#[derive(Debug)]
pub enum InitError {
    /// IO error.
    Io(io::Error),
    /// The log crate's global logger was already initialized when trying to initialize a logger.
    SetLoggerError(log::SetLoggerError),
}

impl convert::From<io::Error> for InitError {
    fn from(error: io::Error) -> InitError {
        InitError::Io(error)
    }
}

impl convert::From<log::SetLoggerError> for InitError {
    fn from(error: log::SetLoggerError) -> InitError {
        InitError::SetLoggerError(error)
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            InitError::Io(ref e) => write!(f, "IO Error initializing logger: {}", e),
            InitError::SetLoggerError(ref e) => write!(f, "logging initialization failed: {}", e),
        }
    }
}

impl error::Error for InitError {
    fn description(&self) -> &str {
        match *self {
            InitError::Io(..) => "IO error while initializing logging",
            InitError::SetLoggerError(..) => "logging system already initialized with different logger",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            InitError::Io(ref e) => Some(e),
            InitError::SetLoggerError(ref e) => Some(e),
        }
    }
}
