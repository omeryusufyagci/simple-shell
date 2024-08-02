# Simple Rust Shell

A simple shell implemented in Rust, capable of executing external commands and handling basic built-in commands.

## Features

- Execute external commands with inherited stdin, stdout, and stderr.
- Built-in commands:
  - `help`: Display help information.
  - `exit`: Exit the shell.
- Handle `CTRL-D` to gracefully exit the shell or logout of a user.

## Usage

To run the shell, compile and execute the program:

```sh
cargo build --release
./target/release/simple_shell
```

## TODO

* (bug) CTRL-C should cancel the current operation and not terminate the shell
* (ft.) Tab completion
* (ft.) PS1 configuration possibility
* (ft.) GUI

## LICENSE

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.