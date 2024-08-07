//! This module is responsible of the core shell functionalities.
//!
//! The `ShellCore` struct manages command execution and maintains the state of any child processes.

use crate::utils::write_output;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(PartialEq)]
pub enum ShellState {
    Running,
    Exiting,
}

pub struct ShellCore {
    pub active_child_process: Arc<Mutex<Option<std::process::Child>>>,
}

/// Manage the execution of commands and handle signals for the shell.
/// Maintain the state of any active child process and provide utility methods
impl ShellCore {
    /// Instantiate a new ShellCore object with no active child process.
    pub fn new() -> Self {
        let shell_core = ShellCore {
            active_child_process: Arc::new(Mutex::new(None)),
        };

        shell_core.setup_signal_handler();

        shell_core
    }

    /// Execute commands based on parsed input.
    /// Returns ShellState for state flow.
    pub fn handle_parsed_input(&self, parsed_input: Vec<&str>) -> ShellState {
        match parsed_input[0] {
            "help" => {
                show_help();
                ShellState::Running
            }
            "exit" => ShellState::Exiting,
            _ => {
                let child_proc = match Command::new(parsed_input[0])
                    .args(&parsed_input[1..])
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn() // returns handler
                {
                    Ok(child_proc) => child_proc,
                    Err(e) => {
                        write_output(("Failed to execute command", e.to_string().as_str()));
                        return ShellState::Running;
                    }
                };

                {
                    // Update active_child_process with current process handle
                    let mut handle_child_proc = self.active_child_process.lock().unwrap();
                    *handle_child_proc = Some(child_proc);
                }

                {
                    // Wait for child and reset
                    let mut handle_child_proc = self.active_child_process.lock().unwrap();
                    if let Some(ref mut child_proc) = *handle_child_proc {
                        child_proc.wait().unwrap();
                    }
                    *handle_child_proc = None;
                }

                ShellState::Running
            }
        }
    }

    /// Sets up a signal handler for `SIGINT` (CTRL-C).
    ///
    /// Spawns a new thread to listen for `SIGINT` signals. Upon detection,
    /// it safely terminates any active child process.
    pub fn setup_signal_handler(&self) {
        let mut signals = Signals::new(&[SIGINT]).unwrap();

        let child_clone = Arc::clone(&self.active_child_process);

        thread::spawn(move || {
            for _ in signals.forever() {
                let mut handle_child_proc = child_clone.lock().unwrap();
                if let Some(ref mut child) = *handle_child_proc {
                    write_output("CTRL-C detected. Terminating active task.\n");
                    child.kill().unwrap();
                } else {
                    write_output("\n-> ");
                }
            }
        });
    }

    /// Run the shell application
    pub fn run_shell(&self, user_input: &mut crate::input_handler::UserInput) {
        loop {
            write_output("-> ");

            let (parsed_input, input_state) = user_input.process_input();

            match input_state {
                crate::input_handler::InputState::Empty => continue,
                crate::input_handler::InputState::Exiting => {
                    write_output("CTRL-D detected. Logging you out...\n");
                    break;
                }
                crate::input_handler::InputState::Valid => {}
            }

            if let Some(parsed_input) = parsed_input {
                if self.handle_parsed_input(parsed_input) == ShellState::Exiting {
                    break;
                }
            }
        }
    }
}

/// Display help
fn show_help() {
    let help_msg = r#"
                            This is a simple shell.
                            Available commands:
                            help - Show this help message
                            exit - Exit the shell
                        "#;

    write_output(help_msg);
}
