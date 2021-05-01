# pykk
[![Downloads](https://pepy.tech/badge/pykk)](https://pepy.tech/project/pykk)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![PyPI version shields.io](https://img.shields.io/pypi/v/pykk.svg)](https://pypi.python.org/pypi/pykk/)

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

In Windows system, you can install with pip command

```bash
pip install pykk
```

MacOS or Linux users should build from source.

```python
import pykk

energy = [1, 2, 3, 4]  # the values MUST have the same intervals
real = [1, 2, 3, 4]

imag = pykk.real2imag(energy, real)
```

## Performance

Compare the performance with the code implemented by Pyhon. The length of the data is ~ 1000 data points.

| Python 3.8 | pykk |
| ---------- | ---- |
| 37 s       | 0.4 ms |


## Licence
This application contains artifacts distributed under the license of the Apache License, Version 2.0.
