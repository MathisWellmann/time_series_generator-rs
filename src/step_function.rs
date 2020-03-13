// step_function returns a vector of given length with step function with mid_point and width
pub fn step_function(length: usize, mid_point: usize, width: usize) -> Vec<f64> {
    let mut out = vec![0.0; length];

    for i in 0..length {
        if i < mid_point - width || i > mid_point + width {
            out[i] = 0.0;
            continue;
        }
        out[i] = 1.0;
    }
    return out
}
