//! This module contains utility functions and traits.
//!
//! It includes the `WriteOutput` trait and related functions for immediate printing and flushing of output.

use std::io::{self, Write};

#[derive(Debug)]
pub enum IoState {
    Ok,
    Error(WriteOutputError),
}

#[derive(Debug)]
pub enum WriteOutputError {
    WriteError,
}

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
    /// "Print immediately!".write_output()?;
    /// ```
    fn write_output(self) -> IoState;
}

/// Implementation of write_output for &str
impl WriteOutput for &str {
    fn write_output(self) -> IoState {
        print!("{}", self);
        match io::stdout().flush() {
            Ok(_) => IoState::Ok,
            Err(_) => IoState::Error(WriteOutputError::WriteError),
        }
    }
}

/// Implementation of write_output for (&str, &str) tuple
/// Typically intended for passing messages with error codes
impl WriteOutput for (&str, &str) {
    fn write_output(self) -> IoState {
        println!("{}: {}", self.0, self.1);
        match io::stdout().flush() {
            Ok(_) => IoState::Ok,
            Err(_) => IoState::Error(WriteOutputError::WriteError),
        }
    }
}

/// Single entry point for write_output
pub fn write_output<T: WriteOutput>(msg: T) -> IoState {
    msg.write_output()
}
