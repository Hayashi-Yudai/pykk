use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;

#[pymodule]
fn pykk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(real2imag))?;
    m.add_wrapped(wrap_pyfunction!(imag2real))?;
    Ok(())
}

#[pyfunction]
fn real2imag(x: &PyList, y: &PyList) -> PyResult<Vec<f64>> {
    const PI: f64 = 3.141592653589;
    let mut result = vec![0.0; y.len()];

    for i in 0..x.len() {
        result[i] = -2.0 / PI * integrate(&x, &y, i);
    }

    Ok(result)
}

#[pyfunction]
fn imag2real(x: &PyList, y: &PyList) -> PyResult<Vec<f64>> {
    const PI: f64 = 3.141592653589;
    let mut result = vec![0.0; y.len()];

    for i in 0..x.len() {
        result[i] = 2.0 / PI * integrate(&x, &y, i);
    }

    Ok(result)
}

fn integrate(x: &PyList, y: &PyList, num: usize) -> f64 {
    let xx: Vec<f64> = x.as_ref().extract().unwrap();
    let yy: Vec<f64> = y.as_ref().extract().unwrap();

    let mut result = 0.0;
    let diff = xx[1] - xx[0];

    for i in 0..x.len() {
        if i == num {
            continue;
        }
        result += xx[num] * yy[i] / (xx[i] * xx[i] - xx[num] * xx[num]) * diff;
    }

    result
}
