use std::sync::{Mutex, Arc};
use std::thread;
use std::f64::consts::PI;

use pyo3::prelude::*;

pub enum Direction {
    REAL2IMAG,
    IMAG2REAL
}

pub fn kk_transform<F>(x: Vec<f64>, y: Vec<f64>, f: F, direction: Direction) -> PyResult<Vec<f64>>
    where F: Fn(&Vec<f64>, &Vec<f64>, usize) -> f64,
          F: Send + Copy + 'static
{
    let thread_num = 16;
    let sign = match direction {
        Direction::REAL2IMAG => 1.0,
        Direction::IMAG2REAL => -1.0,
    };
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
                *val = sign * 2.0 / PI * f(&x, &y, j);
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

/// The main part of the Kramers-Kronig transform from real part to imaginary part.
/// The argument `x` should have narrow enough steps and
/// the space should be equal.
#[allow(dead_code)]
pub fn real2imag_helper(x: &Vec<f64>, y: &Vec<f64>, num: usize) -> f64 {
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
#[allow(dead_code)]
pub fn imag2real_helper(x: &Vec<f64>, y: &Vec<f64>, num: usize) -> f64 {
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
