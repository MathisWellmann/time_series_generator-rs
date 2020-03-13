pub fn sine_wave(length: usize) -> Vec<f64> {
    let mut out = vec![0.0; length];

    for i in 0..length {
        out[i] = 0.99 * ((i as f64 / length as f64) * 2.0 * std::f64::consts::PI).sin()
    }
    return out
}