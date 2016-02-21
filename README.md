disque-cli
====
Disque CLI.

[![Crates version][version-image]][version-url]
[![Build Status][travis-image]][travis-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Crates downloads][downloads-image]][downloads-url]

### Build

```sh
git clone https://github.com/iorust/disque-cli.git && cd disque-cli && cargo build --release
```

### Run

```sh
target/release/disque-cli -h 127.0.0.1 -p 7711
```

More help:
```sh
target/release/disque-cli --help
```

## Use as a crate

```rust
extern crate disque_cli;
// exports:
use disque_cli::{create_client, Client, COMMANDS, Value, encode_slice, Decoder};
```

#### Value, encode_slice, Decoder
Re-exports from the https://github.com/iorust/resp

#### `fn create_client(host: &str, port: u16, password: &str) -> io::Result<Client>`
```Rust
let mut client = create_client("127.0.0.1", 7711, "").expect("Failed to connect");
client.cmd(&["hello"]).unwrap();
```

### Client
```Rust
struct Client {
    // some fields omitted
}
```

#### impl Client

##### `fn new<A: ToSocketAddrs>(addrs: A) -> Self`
```Rust
let mut client = Client::new((hostname, port));
```

##### `fn cmd(&mut self, slice: &[&str]) -> Result<Value>`
```Rust
client.cmd(&["addjob", "test", "hello, world!", "100"]).unwrap(); // Value::String("hello!")
```

##### `fn read_more(&mut self) -> Result<Value>`
Some commands will have one more replies. This method use to read them.
```Rust
client.read_more().unwrap();
```

### COMMANDS
https://github.com/iorust/disque-cli/blob/master/src/command.rs

[version-image]: https://img.shields.io/crates/v/disque-cli.svg
[version-url]: https://crates.io/crates/disque-cli

[travis-image]: http://img.shields.io/travis/iorust/disque-cli.svg
[travis-url]: https://travis-ci.org/iorust/disque-cli

[coveralls-image]: https://coveralls.io/repos/github/iorust/disque-cli/badge.svg?branch=master
[coveralls-url]: https://coveralls.io/github/iorust/disque-cli?branch=master

[downloads-image]: https://img.shields.io/crates/d/disque-cli.svg
[downloads-url]: https://crates.io/crates/disque-cli
