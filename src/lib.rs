mod kk;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;

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
    let thread_num = 16;
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut result: Arc<Vec<Mutex<f64>>> = Arc::new(
        vec![0.0; y.len()]
            .into_iter()
            .map(|x| Mutex::new(x))
            .collect()
    );

    for i in 0..thread_num {
        let x = x.clone();
        let y = y.clone();

        let result = Arc::clone(&mut result);
        let handle = thread::spawn(move || {
            for j in x.len()*i/thread_num..x.len()*(i+1)/thread_num {
                let mut val = result[j].lock().unwrap();
                *val = -2.0 / PI * real2imag_helper(&x, &y, j);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = Arc::try_unwrap(result).unwrap();

    let mut output = vec![0.0; y.len()];
    for (i, val) in result.iter().enumerate() {
        output[i] = *val.lock().unwrap();
    }

    Ok(output)
}

/// Calculate the real part from the imaginary part
///
/// * `x` - independent variable (ex. energy or frequency)
/// * `y` - dependent variable (ex. conductivity or permittivity)
#[pyfunction]
fn imag2real(x: Vec<f64>, y: Vec<f64>) -> PyResult<Vec<f64>> {
    let thread_num = 16;
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut result: Arc<Vec<Mutex<f64>>> = Arc::new(
        vec![0.0; y.len()]
            .into_iter()
            .map(|x| Mutex::new(x))
            .collect()
    );

    for i in 0..thread_num {
        let x = x.clone();
        let y = y.clone();

        let result = Arc::clone(&mut result);
        let handle = thread::spawn(move || {
            for j in x.len()*i/thread_num..x.len()*(i+1)/thread_num {
                let mut val = result[j].lock().unwrap();
                *val = 2.0 / PI * imag2real_helper(&x, &y, j);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = Arc::try_unwrap(result).unwrap();

    let mut output = vec![0.0; y.len()];
    for (i, val) in result.iter().enumerate() {
        output[i] = *val.lock().unwrap();
    }

    Ok(output)
}

