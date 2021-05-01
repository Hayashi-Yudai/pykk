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
