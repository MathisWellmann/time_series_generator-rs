/// Generates a new sine wave function of given length with exactly one period
pub fn generate_sine_wave(length: usize) -> Vec<f64> {
    let mut out = vec![0.0; length];

    for i in 0..length {
        out[i] = 0.99 * ((i as f64 / length as f64) * 2.0 * std::f64::consts::PI).sin()
    }
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot_values;

    #[test]
    fn sine_wave_plot() -> Result<(), Box<dyn std::error::Error>> {
        let sine = generate_sine_wave(256);

        let filename = "img/sine_wave.png";
        plot_values(sine, filename)
    }
}
