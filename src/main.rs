//! The entry point of the shell application.
//!
//! Here the shell is initialized, signal handlers are setup, and the application loop is run.

mod input_handler;
mod shell_core;
mod utils;

use input_handler::UserInput;
use shell_core::ShellCore;

fn main() {
    let mut user_input = UserInput::new();

    let shell = ShellCore::new();
    shell.start_shell(&mut user_input);
}
