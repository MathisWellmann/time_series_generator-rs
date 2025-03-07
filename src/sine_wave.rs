use num_traits::Float;

/// Generates a new sine wave function of given length with exactly one period
pub fn generate_sine_wave<T: Float>(length: usize) -> Vec<T> {
    let mut out = vec![T::zero(); length];

    for (i, val) in out.iter_mut().enumerate() {
        *val = T::from(0.99).expect("Can convert")
            * ((T::from(i).expect("Can convert") / T::from(length).expect("Can convert"))
                * T::from(2.0).expect("Can convert")
                * T::from(std::f64::consts::PI).expect("Can convert"))
            .sin()
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
