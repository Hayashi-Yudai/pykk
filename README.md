# pykk
[![Downloads](https://pepy.tech/badge/pykk)](https://pepy.tech/project/pykk)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![PyPI version shields.io](https://img.shields.io/pypi/v/pykk.svg)](https://pypi.python.org/pypi/pykk/)
[![Test](https://github.com/Hayashi-Yudai/pykk/actions/workflows/CI.yml/badge.svg)](https://github.com/Hayashi-Yudai/pykk/actions/workflows/CI.yml)

Python library for calculating Kramers-Kronig transform written in Rust.

## Requirements

- Python >=3.9,< 3.13

## Install
If you use Windows or Linux, you can install with pip command.

```bash
$ pip install pykk
```

If not, you can install Rust and build from the source.

## build

If you want to build from source, you need maturin. I prepare virtual environment with poetry for building.

```bash
$ poetry install
$ poetry run maturin build --release
```

You can see `.whl` file in `$PROJECT_ROOT/target/wheels`. Install it by pip command.

## How to use

This library has two functions for calculating Kramers-Kronig transform, the transformation from real to imaginary part and vice versa.

```python
import pykk

energy = [1, 2, 3, 4]  # the values MUST have the same intervals
real = [1, 2, 3, 4]

imag = pykk.real2imag(energy, real)  # real -> imaginary part
real_kk = pykk.imag2real(energy, imag)  # imaginary -> real part
```

## Performance

Compare the performance with the code implemented by Python. The length of the data is ~ 1000 data points.

| Python 3.8 | pykk |
| ---------- | ---- |
| 37 s       | 0.4 ms |


## License
This application contains artifacts distributed under the license of the Apache License, Version 2.0.
