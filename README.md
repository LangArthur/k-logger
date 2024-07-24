# K-logger

A little key-logger written in rust for linux. It listens directly to a keyboard input (in /dev/input). At the moment, it does not support multiple keyboards.
This project is still work in progress.

## Build and run

I highly recommend to use cargo to build the code.

```sh
cargo build --release && sudo ./target/release/k-logger
```

:warning: The binary need to be launch with sudo privileges.

## Disclaimer

This project is build for fun and educational purpose. DO NOT USE IT for any illegal actions.

## Built With

* [tracing](https://docs.rs/tracing/latest/tracing/) - tracing is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information.
* [tracing-subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/) - Utilities for implementing and composing tracing subscribers.

## TO-DO list

* [ ] all keys implementation
* [ ] multi-keyboard support
* [ ] international keyboard support
* [ ] internet linked endpoint
* [ ] hide process
* [ ] remove sudo privilege ?
* [ ] Windows support
* [ ] time limitation where the logger stops and delete himself
