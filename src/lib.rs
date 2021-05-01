mod kk;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::f64::consts::PI;

use kk::kk::{real2imag_helper, imag2real_helper};

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
    let mut result = vec![0.0; y.len()];

    for i in 0..x.len() {
        result[i] = -2.0 / PI * real2imag_helper(&x, &y, i);
    }

    Ok(result)
}

/// Calculate the real part from the imaginary part
///
/// * `x` - independent variable (ex. energy or frequency)
/// * `y` - dependent variable (ex. conductivity or permittivity)
#[pyfunction]
fn imag2real(x: Vec<f64>, y: Vec<f64>) -> PyResult<Vec<f64>> {
    let mut result = vec![0.0; y.len()];

    for i in 0..x.len() {
        result[i] = 2.0 / PI * imag2real_helper(&x, &y, i);
    }

    Ok(result)
}

