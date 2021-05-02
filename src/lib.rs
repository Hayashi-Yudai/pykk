mod kk;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use kk::kk::{real2imag_helper, imag2real_helper, kk_transform, Direction};

#[pymodule]
fn pykk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(real2imag))?;
    m.add_wrapped(wrap_pyfunction!(imag2real))?;
    Ok(())
}


/// Calculate the imaginary part from the real part
///
/// * `x` - independent variable (ex. energy or frequency)
/// * `y` - dependent variable (ex. conductivity or permittivity)
#[pyfunction]
fn real2imag(x: Vec<f64>, y: Vec<f64>) -> PyResult<Vec<f64>> {
    let direction = Direction::REAL2IMAG;
    kk_transform(x, y, real2imag_helper, direction)
}

/// Calculate the real part from the imaginary part
///
/// * `x` - independent variable (ex. energy or frequency)
/// * `y` - dependent variable (ex. conductivity or permittivity)
#[pyfunction]
fn imag2real(x: Vec<f64>, y: Vec<f64>) -> PyResult<Vec<f64>> {
    let direction = Direction::IMAG2REAL;
    kk_transform(x, y, imag2real_helper, direction)
}

