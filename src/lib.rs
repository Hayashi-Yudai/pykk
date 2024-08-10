mod kk;

use pyo3::prelude::*;

use kk::kk::{real2imag_helper, imag2real_helper, kk_transform};

#[pymodule]
fn pykk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_function(wrap_pyfunction!(real2imag, m)?);
    let _ = m.add_function(wrap_pyfunction!(imag2real, m)?);

    Ok(())
}


/// Calculate the imaginary part from the real part
///
/// * `x` - independent variable (ex. energy or frequency)
/// * `y` - dependent variable (ex. conductivity or permittivity)
#[pyfunction]
fn real2imag(x: Vec<f64>, y: Vec<f64>) -> PyResult<Vec<f64>> {
    kk_transform(x, y, real2imag_helper)
}

/// Calculate the real part from the imaginary part
///
/// * `x` - independent variable (ex. energy or frequency)
/// * `y` - dependent variable (ex. conductivity or permittivity)
#[pyfunction]
fn imag2real(x: Vec<f64>, y: Vec<f64>) -> PyResult<Vec<f64>> {
    kk_transform(x, y, imag2real_helper)
}

