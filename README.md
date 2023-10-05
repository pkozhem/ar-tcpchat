# Async TCP chat in Rust

Provides to send messages through tcp socket. Only you need is to be connected to single host and port.

## Installation
Clone this project and fill envs in .env file. Then

```bash
cargo run
```

## Work Example
Start several telnet clients on your host and port you provided in env and start typing messages
```bash
telnet 127.0.0.1 8080
```
