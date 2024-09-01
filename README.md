# Simple Rust Shell

A simple shell implemented in Rust, capable of executing external commands and handling basic built-in commands.

## Features

- Execute external commands with inherited stdin, stdout, and stderr.
- Currently available built-in commands:
  - `help`: Display help information.
  - `exit`: Exit the shell.
- Handle `CTRL-C` to cancel the current operation without terminating the shell.
- Handle `CTRL-D` to gracefully exit the shell or logout of a user.

Built-in commands are easily extensible, but current focus is on code design and ensuring basic functionality.

## Installation

To clone and build the project, run the following commands:

```sh
git clone https://github.com/omeryusufyagci/simple-shell.git
cd simple-shell
cargo build --release
```

## Usage
```sh
./target/release/simple_shell
```

## TODO

* (bug) Inconsistent output display and occasional hangs
* (test) Add tests
* (feat) Tab completion
* (feat) History
* (feat) PS1 configuration possibility
* ~~(bug)  Improve error handling; eliminate unwrap usage~~
* ~~(refactor) Modularize the code~~
* ~~(docs) Update docstrings with public doc '///'~~
* ~~(bug) CTRL-C should cancel the current operation and not terminate the shell~~


## LICENSE

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.