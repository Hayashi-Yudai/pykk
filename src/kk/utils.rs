pub fn has_same_interval(x: &Vec<f64>) -> bool {
    let interval = x[1] - x[0];
    if interval == 0.0 {
        return false;
    }

    for i in 1..x.len() {
        if (x[i] - x[i-1] - interval).abs() / interval > 1e-8{
            return false;
        }
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_same_interval() {
        let mut same_interval: Vec<f64> = vec![0.0; 10];
        for i in 0..same_interval.len() {
            same_interval[i] = i as f64;
        }

        assert_eq!(has_same_interval(&same_interval), true);
    }

    #[test]
    fn test_false_in_diff_interval() {
        let mut different_interval = vec![0.0; 10];
        for i in 0..different_interval.len() {
            different_interval[i] = i as f64;
        }
        different_interval[5] -= 1e-5;

        assert_eq!(has_same_interval(&different_interval), false);
    }

    #[test]
    fn test_same_element_vector_return_false() {
        let different_interval = vec![0.0; 10];

        assert_eq!(has_same_interval(&different_interval), false);
    }

    #[test]
    fn allow_diff_smaller_than_one_in_a_million_percent() {
        let mut same_interval = vec![0.0; 10];
        for i in 0..same_interval.len() {
            same_interval[i] = i as f64;
        }

        same_interval[5] -= 1e-8;

        assert_eq!(has_same_interval(&same_interval), true);
    }
}
