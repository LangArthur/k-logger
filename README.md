# K-logger

A little key-logger written in rust for linux. It listens directly to a keyboard input (in /dev/input).

Most of the common keys are supported.

Qwerty and Azerty keyboards are implemented but more keyboard layout can be added to the list.

## Usage

```txt
Usage: k-logger [OPTIONS]

Options:
  -f, --file <FILE>  
  -h, --help         Print help
  -V, --version      Print version
```

:warning: This project is still work in progress.

## Build and run

I highly recommend to use cargo to build the code.

```sh
cargo build --release && sudo ./target/release/k-logger
```

:warning: The binary need to be launch with sudo privileges.

## Built With

* [clap](https://docs.rs/clap/latest/clap/index.html) - Command Line Argument Parser for Rust.
* [tracing](https://docs.rs/tracing/latest/tracing/) - tracing is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information.
* [tracing-subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/) - Utilities for implementing and composing tracing subscribers.

## TO-DO list

* [x] Common keys implementation
* [x] multi-keyboard support
* [x] international keyboard support (Azerty + Qwerty)
* [x] write inputs to a file
* [ ] internet linked endpoint
* [ ] hide process
* [ ] remove sudo privilege ?
* [ ] Windows support
* [ ] time limitation where the logger stops and delete himself

## Disclaimer

This project is build for fun and educational purpose. DO NOT USE IT for any illegal actions.
