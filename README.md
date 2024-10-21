# Simple Rust Shell

This was a simple test to get a feel for how CLI tooling works with Rust. It's a simple shell that supports extensible arguments. 

## Features

- Execute external commands with inherited stdin, stdout, and stderr.
- Support for built-in commands such as `help` or `exit`.
- Graceful signal handling for `CTRL-C` to cancel operation and `CTRL-D` for user logout.

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

## LICENSE

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
