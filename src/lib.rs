use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use num_traits::{NumAssign, NumCast};

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

fn integrate<T>(x: &Vec<T>, y: &Vec<T>, num: usize) -> f64
where
    T: NumAssign + NumCast + Copy,
{
    let mut result = T::from(0.0).unwrap();
    let diff = x[1] - x[0];

    for i in 0..x.len() {
        if i == num {
            continue;
        }
        result += x[num] * y[i] / (x[i] * x[i] - x[num] * x[num]) * diff;
    }

    result.to_f64().unwrap()
}
