#![deny(missing_docs)]
#![doc(html_root_url = "https://dabo.guru/rust/fern/")]
//! Efficient, configurable logging in rust
//!
//! With Fern, you can:
//!
//! - Configure logging at runtime; make changes based off of user arguments or configuration
//! - Format log records without allocating intermediate results
//! - Output to stdout, stderr, log files and custom destinations
//! - Apply a blanket level filter and per-crate/per-module overrides
//! - Intuitively apply filters and formats to groups of loggers via builder chaining
//! - Log using the standard `log` crate macros
//!
//! Fern, while feature-complete, does not have a mature API. The library may be changed
//! in backwards incompatible ways to make it more ergonomic in the future.
//!
//! Depending on Fern
//! =================
//!
//! To use fern effectively, depend on the `fern` and `log` crates in your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! log = "0.3"
//! fern = "0.4"
//! ```
//!
//! Then declare as an extern crates at your program's root file:
//!
//! ```
//! // main.rs or lib.rs:
//! #[macro_use]
//! extern crate log;
//! extern crate fern;
//! # fn main() {}
//! ```
//!
//! Example usage:
//! ==============
//!
//! In fern 0.4, creating, configuring, and establishing a logger as the global logger are all merged
//! into builder methods on the `Dispatch` struct.
//!
//! Here's an example logger which formats messages, limits to Debug level, and puts everything into both stdout and
//! an output.log file.
//!
//! ```no_run
//! # // no_run because this creates an output.log file.
//! extern crate fern;
//! extern crate log;
//! extern crate time;
//!
//! fern::Dispatch::new()
//!     .format(|output, message, record| {
//!         use ::std::fmt::Write;
//!
//!         write!(output,
//!                "[{}][{}][{}] {}",
//!                time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(),
//!                record.target(),
//!                record.level(),
//!                message)
//!     })
//!     .level(log::LogLevelFilter::Debug)
//!     .chain(std::io::stdout())
//!     .chain(fern::log_file("output.log").expect("failed to open log file"))
//!     .set_global()
//!     .expect("global logger already initialized");
//! ```
//!
//! Let's unwrap the above example:
//!
//! ### `fern::Dispatch::new()`
//!
//! Create an empty logger config.
//!
//! ### `.format(|...| ...)`
//!
//!
//! Add a formatter to the logger, modifying all messages sent through.
//!
//! ### `time::now().strftim(...)`
//!
//!
//! This uses the [`time`] library to make a readable string. The final output of the format will be like:
//!
//! ```text
//! [2015-01-20][12:55:04][crate-name][INFO] Something happened.
//! ```
//!
//! ### `.level(log::LogLevelFilter::Debug)`
//!
//! Set the minimum level needed to output to Debug, accepting Debug, Info, Warn, and Error-level messages
//! and denying Trace-level messages.
//!
//! ### `.chain(std::io::stdout())`
//!
//! Add a child to the logger; send all messages to stdout.
//!
//! `.chain()` accepts Stdout, Stderr, Files and other Dispatch instances.
//!
//! ### `.chain(fern::log_file(...).expect(...))`
//!
//! Add a second child; send all messages to the file "output.log".
//!
//! `fern::log_file()` is simply a convenience method equivalent to:
//!
//! ```no_run
//! # use std::fs::OpenOptions;
//! # drop(
//! OpenOptions::new()
//!     .write(true)
//!     .create(true)
//!     .append(true)
//!     .open("filename")
//! # )
//! ```
//!
//! ### `.set_global()`
//!
//! Consume the Dispatch instance, create a `log`-crate logger, and instantiate it as the current runtime's logger.
//!
//! This will fail if and only if another fern or `log` logger has already been set as the global logger.
//!
//! Logging
//! ===
//!
//! Once the logger has been set using set_global, it will pick up all `log`-crate log calls from your crate and
//! all your libraries.
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//! extern crate fern;
//!
//! # fn main() {
//! fern::Dispatch::new()
//!     // ...
//!     .set_global();
//!
//! trace!("Trace message");
//! debug!("Debug message");
//! info!("Info message");
//! warn!("Warning message");
//! error!("Error message");
//! # }
//! ```
//!
//! More configuration
//! ===
//!
//! The example above annotates a a basic configuration, but more is allowed! The documentation for `Dispatch` contains
//! a complete set of configurations, and an example using all possible options.
//!
//! [time]: https://crates.io/crates/time
//! [time-docs]: https://doc.rust-lang.org/time/time/index.html

extern crate log;

use std::convert::AsRef;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::{io, fmt};

pub use builders::{Dispatch, Output};
pub use errors::InitError;

mod builders;
mod log_impl;
mod errors;

/// A type alias for a log formatter.
pub type Formatter = Fn(&mut fmt::Write, &fmt::Arguments, &log::LogRecord) -> fmt::Result + Sync + Send;

/// A type alias for a log filter. Returning true means the record should succeed - false means it should fail.
pub type Filter = Fn(&log::LogMetadata) -> bool + Send + Sync;

/// Fern logging trait. This is necessary in order to allow for custom loggers taking in arguments that have already had
/// a custom format applied to them.
///
/// The original `log::Log` trait's `log` method only accepts messages that were created using the log macros - this
/// trait also accepts records which have had additional formatting applied to them.
pub trait FernLog: Sync + Send {
    /// Logs a log record, but with the given fmt::Arguments instead of the one contained in the LogRecord.
    ///
    /// This has access to the original record, but _should ignore_ the original `record.args()` and instead
    /// use the passed in payload.
    fn log_args(&self, payload: &fmt::Arguments, record: &log::LogRecord);
}

/// Convenience method for opening a log file with common options.
///
/// Exactly equivalent to:
///
/// ```no_run
/// # use std::fs::OpenOptions;
/// OpenOptions::new()
///     .write(true)
///     .create(true)
///     .append(true)
///     .open("filename")
/// # ;
/// ```
#[inline]
pub fn log_file<P: AsRef<Path>>(path: P) -> Result<File, io::Error> {
    OpenOptions::new().write(true).create(true).append(true).open(path)
}
