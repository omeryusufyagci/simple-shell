use signal_hook::{consts::SIGINT, iterator::Signals};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(PartialEq)]
enum ShellState {
    Running,
    Exiting,
}

#[derive(PartialEq)]
enum InputState {
    Valid,
    Empty,
    Exiting,
}

trait PrintAndFlush {
    /*
     * Interface for immediate printing.
     * Implementations overload the print_and_flush method
     */

    fn print_and_flush(self) -> io::Result<()>;
}

impl PrintAndFlush for &str {
    /*
     * Implementation of print_and_flush for &str
     */

    fn print_and_flush(self) -> io::Result<()> {
        print!("{}", self);
        io::stdout().flush()
    }
}

impl PrintAndFlush for (&str, &str) {
    /*
     * Implementation of print_and_flush for (&str, &str) tuple
     * Typically intended for passing messages with error codes
     */

    fn print_and_flush(self) -> io::Result<()> {
        println!("{}: {}", self.0, self.1);
        io::stdout().flush()
    }
}

fn print_and_flush<T: PrintAndFlush>(msg: T) -> io::Result<()> {
    /*
     * Single entry point for print_and_flush
     */

    msg.print_and_flush()
}

fn read_and_parse_input() -> (Option<Vec<String>>, InputState) {
    /*
     * Read user input and parse it into a vector of Strings
     * Return an Optional parsed_input and the InputState
     *
     * TODO: Adapt to use slices while keeping the encapsulation
     * need to properly manage lifetimes; stick to owned data
     * in the meantime with Strings.
     */

    let mut user_input = String::new();
    let read_input = match io::stdin().read_line(&mut user_input) {
        Ok(n) => n,
        Err(_) => return (None, InputState::Exiting),
    };
    // Read input is 0 when CTRL-D is pressed
    if read_input == 0 {
        return (None, InputState::Exiting);
    }

    let trimmed_input = user_input.trim();
    let parsed_input: Vec<String> = trimmed_input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let input_state: InputState = if parsed_input.is_empty() {
        InputState::Empty
    } else {
        InputState::Valid
    };

    (Some(parsed_input), input_state)
}

fn handle_parsed_input(
    parsed_input: Vec<String>,
    active_child_process: &Arc<Mutex<Option<std::process::Child>>>,
) -> ShellState {
    /* Handle specific and generic implementations of commands and signals
     * Return the ShellState for state machine
     */

    match parsed_input[0].as_str() {
        "help" => {
            show_help();
            ShellState::Running
        }
        "exit" => ShellState::Exiting,
        _ => {
            let child_proc = match Command::new(&parsed_input[0])
                .args(&parsed_input[1..])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn() // returns handler
            {
                Ok(child_proc) => child_proc,
                Err(e) => {
                    print_and_flush(("Failed to execute command", e.to_string().as_str())).unwrap();
                    return ShellState::Running;
                }
            };

            {
                // Update active_child_process with current process handle
                let mut handle_child_proc = active_child_process.lock().unwrap();
                *handle_child_proc = Some(child_proc);
            }

            {
                // Wait for child and reset
                let mut handle_child_proc = active_child_process.lock().unwrap();
                if let Some(ref mut child_proc) = *handle_child_proc {
                    child_proc.wait().unwrap();
                }
                *handle_child_proc = None;
            }

            ShellState::Running
        }
    }
}

fn setup_signal_handler(active_child_process: Arc<Mutex<Option<std::process::Child>>>) {
    /*
     * Setup a signal handler for SIGINT (CTRL-C).
     * Spawn a new thread to listen for SIGINT signals.
     * Upon detection, safely kill the child process.
     */

    let mut signals = Signals::new(&[SIGINT]).unwrap();

    let child_clone = Arc::clone(&active_child_process);

    thread::spawn(move || {
        for _ in signals.forever() {
            let mut handle_child_proc = child_clone.lock().unwrap();
            if let Some(ref mut child) = *handle_child_proc {
                print_and_flush("CTRL-C detected. Terminating active task.\n").unwrap();
                child.kill().unwrap();
            } else {
                print_and_flush("\n-> ").unwrap();
            }
        }
    });
}

fn show_help() {
    /*
     * Display help
     */

    let help_msg = r#"
                        This is a simple shell.
                        Available commands:
                        help - Show this help message
                        exit - Exit the shell
                        "#;
    print_and_flush(help_msg).unwrap();
}

fn main() {
    let active_child_process = Arc::new(Mutex::new(None));
    setup_signal_handler(Arc::clone(&active_child_process));

    loop {
        print_and_flush("-> ").unwrap();

        let (parsed_input, input_state) = read_and_parse_input();

        match input_state {
            InputState::Empty => continue,
            InputState::Exiting => {
                print_and_flush("CTRL-D detected. Logging you out...\n").unwrap();
                break;
            }
            InputState::Valid => {}
        }

        if let Some(parsed_input) = parsed_input {
            if handle_parsed_input(parsed_input, &active_child_process) == ShellState::Exiting {
                break;
            }
        }
    }
}
