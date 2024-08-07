//! This module contains utility functions and traits.
//!
//! It includes the `WriteOutput` trait and related functions for immediate printing and flushing of output.

use std::io::{self, Write};

/// Interface for immediate printing.
/// Implementations define the behaviour of write_output
pub trait WriteOutput {
    /// Prints and flushes the output.
    ///
    /// # Usage
    ///
    /// ```
    /// use crate::utils::WriteOutput;
    ///
    /// "Print immediately!".write_output().unwrap();
    /// ```
    fn write_output(self) -> io::Result<()>;
}

/// Implementation of write_output for &str
impl WriteOutput for &str {
    fn write_output(self) -> io::Result<()> {
        print!("{}", self);
        io::stdout().flush()
    }
}

/// Implementation of write_output for (&str, &str) tuple
/// Typically intended for passing messages with error codes
impl WriteOutput for (&str, &str) {
    fn write_output(self) -> io::Result<()> {
        println!("{}: {}", self.0, self.1);
        io::stdout().flush()
    }
}

/// Single entry point for write_output
pub fn write_output<T: WriteOutput>(msg: T) -> io::Result<()> {
    msg.write_output()
}
