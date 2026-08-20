import numpy as np
import pytest

import pykk


def reference_transform(x, y, kind):
    """Straightforward Python implementation of the discrete KK transform.

    Mirrors the summation performed by the Rust core so that the numerical
    contract of the extension can be pinned down from the Python side.
    """
    x = np.asarray(x, dtype=np.float64)
    y = np.asarray(y, dtype=np.float64)
    diff = x[1] - x[0]

    out = np.zeros(len(y), dtype=np.float64)
    for n in range(len(x)):
        base = x[n]
        total = 0.0
        for xx, yy in zip(x, y):
            if xx == base:
                continue
            if kind == "real2imag":
                total -= base * yy / (xx * xx - base * base) * diff
            else:
                total += xx * yy / (xx * xx - base * base) * diff
        out[n] = 2.0 / np.pi * total

    return out


@pytest.fixture
def sample():
    x = np.linspace(1.0, 10.0, 64)
    y = 1.0 / (1.0 + (x - 5.0) ** 2)
    return x, y


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_returns_float64_ndarray_for_list_input(func):
    result = func([1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0])

    assert isinstance(result, np.ndarray)
    assert result.dtype == np.float64
    assert result.shape == (4,)


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_accepts_float64_ndarray(func, sample):
    x, y = sample

    result = func(x, y)

    assert isinstance(result, np.ndarray)
    assert result.shape == y.shape


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_list_and_ndarray_inputs_agree(func, sample):
    x, y = sample

    from_ndarray = func(x, y)
    from_list = func(x.tolist(), y.tolist())

    np.testing.assert_array_equal(from_ndarray, from_list)


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_accepts_integer_dtype_ndarray(func):
    x = np.arange(1, 21, dtype=np.int64)
    y = np.arange(1, 21, dtype=np.int64)

    result = func(x, y)

    np.testing.assert_allclose(
        result, func(x.astype(np.float64), y.astype(np.float64)), rtol=0, atol=0
    )


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_accepts_float32_ndarray(func):
    # Values are chosen so that they are exact in float32: the uniform-interval
    # check tolerates only a 1e-8 relative deviation, which a float32 grid with
    # an inexact step could not satisfy.
    x = np.arange(1.0, 21.0, dtype=np.float32)
    y = (1.0 / x).astype(np.float32)

    result = func(x, y)

    np.testing.assert_allclose(result, func(x.astype(np.float64), y.astype(np.float64)))


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_accepts_non_contiguous_ndarray(func):
    x = np.arange(1.0, 41.0)[::2]
    y = np.sin(np.arange(1.0, 41.0))[::2]

    assert not x.flags["C_CONTIGUOUS"]

    result = func(x, y)

    np.testing.assert_array_equal(result, func(np.ascontiguousarray(x), np.ascontiguousarray(y)))


@pytest.mark.parametrize("kind", ["real2imag", "imag2real"])
def test_matches_reference_implementation(kind, sample):
    x, y = sample

    result = getattr(pykk, kind)(x, y)

    np.testing.assert_allclose(result, reference_transform(x, y, kind), rtol=1e-12)


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_non_uniform_x_raises_value_error(func):
    x = np.array([1.0, 2.0, 4.0, 8.0])
    y = np.array([1.0, 1.0, 1.0, 1.0])

    with pytest.raises(ValueError, match="same interval"):
        func(x, y)


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_length_mismatch_raises_value_error(func):
    x = np.arange(1.0, 6.0)
    y = np.arange(1.0, 4.0)

    with pytest.raises(ValueError):
        func(x, y)


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_too_short_input_raises_value_error(func):
    with pytest.raises(ValueError):
        func([1.0], [1.0])


@pytest.mark.parametrize("func", [pykk.real2imag, pykk.imag2real])
def test_two_dimensional_input_is_rejected(func):
    x = np.arange(1.0, 9.0).reshape(2, 4)
    y = np.arange(1.0, 9.0).reshape(2, 4)

    with pytest.raises((TypeError, ValueError)):
        func(x, y)
