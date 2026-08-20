mod kk;

use numpy::{AllowTypeChange, IntoPyArray, PyArray1, PyArrayLike1};
use pyo3::prelude::*;

use kk::kk::{real2imag_helper, imag2real_helper, kk_transform};

#[pymodule]
fn pykk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    let _ = m.add_function(wrap_pyfunction!(real2imag, m)?);
    let _ = m.add_function(wrap_pyfunction!(imag2real, m)?);

    Ok(())
}


/// Calculate the imaginary part from the real part
///
/// * `x` - independent variable (ex. energy or frequency)
/// * `y` - dependent variable (ex. conductivity or permittivity)
///
/// Both arguments accept anything `numpy.asarray` can turn into a 1-D array
/// of floats (lists, tuples, and ndarrays of any numeric dtype). The result
/// is always a `numpy.ndarray` of `float64`.
#[pyfunction]
fn real2imag<'py>(
    py: Python<'py>,
    x: PyArrayLike1<'py, f64, AllowTypeChange>,
    y: PyArrayLike1<'py, f64, AllowTypeChange>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let result = kk_transform(x.as_array().to_vec(), y.as_array().to_vec(), real2imag_helper)?;

    Ok(result.into_pyarray(py))
}

/// Calculate the real part from the imaginary part
///
/// * `x` - independent variable (ex. energy or frequency)
/// * `y` - dependent variable (ex. conductivity or permittivity)
///
/// Both arguments accept anything `numpy.asarray` can turn into a 1-D array
/// of floats (lists, tuples, and ndarrays of any numeric dtype). The result
/// is always a `numpy.ndarray` of `float64`.
#[pyfunction]
fn imag2real<'py>(
    py: Python<'py>,
    x: PyArrayLike1<'py, f64, AllowTypeChange>,
    y: PyArrayLike1<'py, f64, AllowTypeChange>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let result = kk_transform(x.as_array().to_vec(), y.as_array().to_vec(), imag2real_helper)?;

    Ok(result.into_pyarray(py))
}
