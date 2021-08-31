/// Generate a triangle wave with a single up and down wave
pub fn generate_triangle_wave(length: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity(length);

    let half_way = length / 2;
    let delta = 2.0 / length as f64;
    let mut val: f64 = 0.0;
    for i in 0..length {
        if i > half_way {
            val -= delta;
        } else {
            val += delta;
        }
        out.push(val);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot_values;

    #[test]
    fn triangle_wave_plot() {
        let trg = generate_triangle_wave(256);

        let filename = "img/triangle_wave.png";
        plot_values(trg, filename).unwrap();
    }
}
