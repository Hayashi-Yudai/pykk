use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::f64::consts::PI;

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

/// The main part of the Kramers-Kronig transform from real part to imaginary part.
/// The argument `x` should have narrow enough steps and
/// the space should be equal.
fn real2imag_helper(x: &Vec<f64>, y: &Vec<f64>, num: usize) -> f64 {
    let mut result = 0.0;
    let diff = x[1] - x[0];

    let base = x[num];

    for (xx, yy) in x.iter().zip(y.iter()) {
        if *xx == base {
            continue;
        }
        result += base * yy / (xx * xx - base * base) * diff;
    }

    result
}

/// The main part of the Kramers-Kronig transform from imaginary part to real part.
/// The argument `x` should have narrow enough steps and
/// the space should be equal.
fn imag2real_helper(x: &Vec<f64>, y: &Vec<f64>, num: usize) -> f64 {
    let mut result = 0.0;
    let diff = x[1] - x[0];

    let base = x[num];

    for (xx, yy) in x.iter().zip(y.iter()) {
        if *xx == x[num] {
            continue;
        }
        result += xx * yy / (xx * xx - base * base) * diff;
    }

    result
}
