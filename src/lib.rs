use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn pykk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(real2imag))?;
    Ok(())
}

#[pyfunction]
fn real2imag(x: Vec<f64>, y: Vec<f64>) -> PyResult<Vec<f64>> {
    const PI: f64 = 3.141592653589;
    let mut result = vec![0.0; y.len()];

    for i in 0..x.len() {
        result[i] = -2.0 / PI * integrate(&x, &y, i);
    }

    Ok(result)
}

fn integrate(x: &Vec<f64>, y: &Vec<f64>, num: usize) -> f64{
    let mut result = 0.0;
    let diff = x[1] - x[0];

    for i in 0..x.len() {
        if i == num {
            continue;
        }
        result += x[num] * y[i] / (x[i]*x[i] - x[num]*x[num]) * diff;
    }

    result
}
