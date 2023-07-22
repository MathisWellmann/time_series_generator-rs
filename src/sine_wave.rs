/// Generates a new sine wave function of given length with exactly one period
pub fn generate_sine_wave(length: usize) -> Vec<f64> {
    let mut out = vec![0.0; length];

    for (i, val) in out.iter_mut().enumerate() {
        *val = 0.99 * ((i as f64 / length as f64) * 2.0 * std::f64::consts::PI).sin()
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot_2d;

    #[test]
    fn sine_wave_plot() -> Result<(), Box<dyn std::error::Error>> {
        let sine = generate_sine_wave(256);

        let filename = "img/sine_wave.png";
        plot_2d(sine, filename)
    }
}
