# pykk

Python library for calculating Kramers-Kronig transform written in Rust

## Requirements

For build:

- Rust 1.50.0

For use:

- Python 3.6+

## build

If you want to build from source, run the following command in your shell

```
$ cargo build --release
```

- Linux

Rename `target/release/libpykk.so` to `target/release/pykk.so`

- Windows

Rename `target/release/pykk.dll` to `target/release/pykk.pyd`

## How to use

In using this package in your Python code, [download binary](https://github.com/Hayashi-Yudai/pykk/releases/tag/v0.1.1)(pykk.so for Linux and pykk.pyd for Windows) and put it on the same directory with your Python code.

```python
import pykk

energy = [1, 2, 3, 4]
real = [1, 2, 3, 4]

imag = pykk.real2imag(energy, real)
```

## Performance

Compare the performance with the code implemented by Pyhon. The length of the data is ~ 1000 data points.

| Python 3.8 | pykk |
| ---------- | ---- |
| 37 s       | 7 ms |
