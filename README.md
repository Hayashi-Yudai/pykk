# pykk

Python library for calculating Kramers-Kronig transform written in Rust

## Requirements
For build:

- Rust 1.50.0

For use:
- Python 3.6+

## build

In your shell, run the following command
```
$ cargo build --release
```

- Linux

Rename `target/release/libpykk.so` to `target/release/pykk.so`

- Windows

Rename `target/release/pykk.dll` to `target/release/pykk.pyd`
