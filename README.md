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

* Tab completion
* GUI

## LICENSE

This project is licensed under the MIT License. See the LICENSE file for details.