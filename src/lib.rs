use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn pykk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_zeros_array))?;
    Ok(())
}

#[pyfunction]
fn get_zeros_array(n: Vec<i32>) -> PyResult<Vec<i32>> {
    Ok(get_ones(n))
}

fn get_ones(n: Vec<i32>) -> Vec<i32> {
    vec![1; n.len()]
}
