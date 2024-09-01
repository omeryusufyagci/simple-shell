//! This module is responsible of the core shell functionalities.
//!
//! The `ShellCore` struct manages command execution and maintains the state of any child processes.

use crate::utils::{write_output, IoState, WriteOutputError};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::fmt;
use std::fmt::write;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

/* TODO:
 * Fix error messages without codes, details
 * See other inline todos
 */

#[derive(PartialEq)]
pub enum ShellState {
    Running,
    Exiting,
}

#[derive(Debug)]
pub enum ShellError {
    WriteError(WriteOutputError),
    LockError(String),
    SignalError(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::WriteError(err) => write!(f, "Write error: {}", err),
            ShellError::LockError(err) => write!(f, "Lock error: {}", err),
            ShellError::SignalError(err) => write!(f, "Signal error: {}", err),
        }
    }
}

impl From<WriteOutputError> for ShellError {
    fn from(error: WriteOutputError) -> Self {
        ShellError::WriteError(error)
    }
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

        if let Err(_) = shell_core.setup_signal_handler() {
            let _ = write_output("Failed to setup signal handler");
        }
        shell_core
    }

    /// Start the application loop
    pub fn start_shell(&self, user_input: &mut crate::input_handler::UserInput) {
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
                if self.dispatch_command(parsed_input) == ShellState::Exiting {
                    break;
                }
            }
        }
    }

    /// Dispatch the command based on the parsed input; execute either a built-in command or a system command.
    fn dispatch_command(&self, parsed_input: Vec<&str>) -> ShellState {
        /* TODO: should we really need to return shell state on dispatch?
         * seems already handle before we reach this point, might not be necessary.
         */

        // Ensure we have at least one element (the command)
        if parsed_input.is_empty() {
            let _ = write_output("No command entered.\n");
            return ShellState::Running;
        }

        // TODO: input_handler should take of this
        let command = parsed_input[0];
        let arguments = &parsed_input[1..];

        match command {
            "help" => {
                self.display_help();
                ShellState::Running
            }
            "exit" => ShellState::Exiting,
            _ => {
                if let Err(e) = self.run_system_command(command, arguments) {
                    let _ = write_output(("Failed to execute command", e.to_string().as_str()));
                }
                ShellState::Running
            }
        }
    }

    fn run_system_command(&self, command: &str, arguments: &[&str]) -> Result<(), ShellError> {
        let child_process = Command::new(command)
            .args(arguments)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| ShellError::SignalError(e.to_string()))?;

        self.set_active_child_process(child_process)?;

        Ok(())
    }

    /// Set the active child process, replacing and cleaning up any existing process.
    fn set_active_child_process(&self, child_proc: std::process::Child) -> Result<(), ShellError> {
        // Clean-up any artifacts (not possible to spawn concurrent user-commands from the same shell instance)
        self.terminate_child_process()?;

        // update handle
        let mut handle_child_process = self.active_child_process.lock().map_err(|_| {
            ShellError::LockError("Failed to acquire lock for child process.".into())
        })?;
        *handle_child_process = Some(child_proc);
        Ok(())
    }

    /// Terminate active child process: wait for it to finish and clear.
    fn terminate_child_process(&self) -> Result<(), ShellError> {
        let mut handle_child_process = self.active_child_process.lock().map_err(|_| {
            ShellError::LockError("Failed to acquire lock for child process.".into())
        })?;

        if let Some(ref mut child) = *handle_child_process {
            if let Err(e) = child.kill() {
                let _ = write_output(("Failed to kill child process", e.to_string().as_str()));
            }
            // TODO: what if it hangs? need a timeout; wait-timeout.rs could be used
            child
                .wait()
                .map_err(|e| ShellError::SignalError(e.to_string()))?;
        }

        *handle_child_process = None;
        Ok(())
    }

    /// Sets up a signal handler for `SIGINT` (CTRL-C).
    ///
    /// Spawns a new thread to listen for `SIGINT` signals. Upon detection,
    /// it safely terminates any active child process.
    fn setup_signal_handler(&self) -> Result<(), ShellError> {
        let mut signals =
            Signals::new(&[SIGINT]).map_err(|e| ShellError::SignalError(e.to_string()))?;

        let child_clone = Arc::clone(&self.active_child_process);

        thread::spawn(move || {
            for _ in signals.forever() {
                let mut handle_child_proc = match child_clone.lock() {
                    Ok(handle) => handle,
                    Err(_) => {
                        let _ = write_output("Failed to acquire lock for signal handler.");
                        continue;
                    }
                };
                if let Some(ref mut child) = *handle_child_proc {
                    let _ = write_output("CTRL-C detected. Terminating active task.\n");
                    if let Err(e) = child.kill() {
                        let _ =
                            write_output(("Failed to kill child process", e.to_string().as_str()));
                    }
                } else {
                    write_output("\n-> ");
                }
            }
        });
        Ok(())
    }

    /// Display help
    fn display_help(&self) {
        let help_msg = r#"
                            This is a simple shell.
                            Available commands:
                            help - Show this help message
                            exit - Exit the shell
                        "#;

        write_output(help_msg);
    }
}
