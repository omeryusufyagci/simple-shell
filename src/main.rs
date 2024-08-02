use std::io::{self, Write};
use std::process::{Command, Stdio};

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

fn read_and_parse_input(user_input: &mut String) -> (Option<Vec<&str>>, InputState) {
    /*
     * Read user input and parse it into a vector of strings
     * Return an Optional parsed_input and the InputState
     */

    let read_input = match io::stdin().read_line(user_input) {
        Ok(n) => n,
        Err(_) => return (None, InputState::Exiting),
    };
    // Read input is 0 when CTRL-D is pressed
    if read_input == 0 {
        return (None, InputState::Exiting);
    }

    let trimmed_input = user_input.trim();
    let parsed_input: Vec<&str> = trimmed_input.split_whitespace().collect();

    let input_state: InputState = if parsed_input.is_empty() {
        InputState::Empty
    } else {
        InputState::Valid
    };

    (Some(parsed_input), input_state)
}

fn handle_parsed_input(parsed_input: Vec<&str>) -> ShellState {
    /* Handle specific and generic implementations of commands
     * Return the ShellState for state machine
     */

    match parsed_input[0] {
        "help" => {
            show_help();
            ShellState::Running
        }
        "exit" => ShellState::Exiting,
        _ => {
            // Execute a new command with args, using parent stds
            match Command::new(parsed_input[0])
                .args(&parsed_input[1..])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
            {
                Ok(_) => {}
                Err(e) => {
                    print_and_flush(("Failed to execute command", e.to_string().as_str())).unwrap();
                }
            }
            ShellState::Running
        }
    }
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
    loop {
        print_and_flush("-> ").unwrap();

        let mut user_input = String::new();

        let (parsed_input, input_state) = read_and_parse_input(&mut user_input);

        match input_state {
            InputState::Empty => continue,
            InputState::Exiting => {
                print_and_flush("CTRL-D detected. Logging you out...\n").unwrap();
                break;
            }
            InputState::Valid => {}
        }

        if let Some(parsed_input) = parsed_input {
            if handle_parsed_input(parsed_input) == ShellState::Exiting {
                break;
            }
        }
    }
}
