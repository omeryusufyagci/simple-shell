//! This module contains utility functions and traits.
//!
//! It includes the `PrintAndFlush` trait and related functions for immediate printing and flushing of output.

use std::io::{self, Write};

/// Interface for immediate printing.
/// Implementations define the behaviour of print_and_flush
pub trait PrintAndFlush {
    /// Prints and flushes the output.
    ///
    /// # Usage
    ///
    /// ```
    /// use crate::utils::PrintAndFlush;
    ///
    /// "Print immediately!".print_and_flush().unwrap();
    /// ```
    fn print_and_flush(self) -> io::Result<()>;
}

/// Implementation of print_and_flush for &str
impl PrintAndFlush for &str {
    fn print_and_flush(self) -> io::Result<()> {
        print!("{}", self);
        io::stdout().flush()
    }
}

/// Implementation of print_and_flush for (&str, &str) tuple
/// Typically intended for passing messages with error codes
impl PrintAndFlush for (&str, &str) {
    fn print_and_flush(self) -> io::Result<()> {
        println!("{}: {}", self.0, self.1);
        io::stdout().flush()
    }
}

/// Single entry point for print_and_flush
pub fn print_and_flush<T: PrintAndFlush>(msg: T) -> io::Result<()> {
    msg.print_and_flush()
}
