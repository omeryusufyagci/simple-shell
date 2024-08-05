# Simple Rust Shell

A simple shell implemented in Rust, capable of executing external commands and handling basic built-in commands.

## Features

- Execute external commands with inherited stdin, stdout, and stderr.
- Currently available built-in commands:
  - `help`: Display help information.
  - `exit`: Exit the shell.
- Handle `CTRL-D` to gracefully exit the shell or logout of a user.

Built-in commands are easily extensible, but current focus is on code design and ensuring basic functionality.

## Usage

To run the shell, compile and execute the program:

```sh
cargo build --release
./target/release/simple_shell
```

## TODO

* (bug)  Use expect instead of unwrap
* (docs) Update docstrings with public doc '///'
* (test) Add tests
* (feat) Tab completion
* (feat) History
* (feat) PS1 configuration possibility
* (feat) GUI
* ~~(bug) CTRL-C should cancel the current operation and not terminate the shell~~


## LICENSE

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.