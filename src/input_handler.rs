//! This module is responsible for handling user inputs.
//!
//! It reads and parses input from stdin and determines the state of the input.

use std::io;

#[derive(PartialEq)]
pub enum InputState {
    Valid,
    Empty,
    Exiting,
}

pub struct UserInput {
    buffer: String,
}

/// Instantiate a new UserInput object with a heap allocated buffer
impl UserInput {
    pub fn new() -> Self {
        UserInput {
            buffer: String::new(),
        }
    }

    /// Read user input and determine its state.
    pub fn process_input(&mut self) -> (Option<Vec<&str>>, InputState) {
        self.buffer.clear();

        let read_input = match io::stdin().read_line(&mut self.buffer) {
            Ok(n) => n,
            Err(_) => return (None, InputState::Exiting),
        };

        // Read input is 0 when CTRL-D is pressed
        if read_input == 0 {
            return (None, InputState::Exiting);
        }

        let trimmed_input = self.buffer.trim();
        let parsed_input: Vec<&str> = trimmed_input.split_whitespace().collect();

        let input_state = if parsed_input.is_empty() {
            InputState::Empty
        } else {
            InputState::Valid
        };

        (Some(parsed_input), input_state)
    }
}
