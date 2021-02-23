use rand::prelude::*;
use rand_distr::StandardNormal;

/// generate a new randomly sampled timeseries of given length
/// using a standard normal distribution scaled down by 100
/// by multiplying a random (+/-) value with the previous value
pub fn generate_standard_normal(length: usize, start_value: f64) -> Vec<f64> {
    let mut out: Vec<f64> = vec![0.0; length];

    let mut rng = thread_rng();

    out.push(start_value);
    let mut last_val: f64 = start_value;
    for v in out.iter_mut() {
        let r: f64 = rng.sample(StandardNormal);
        *v = last_val + (last_val * (r / 100.0));
        last_val = *v;
    }

    return out;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot_values;

    #[test]
    fn generate_standard_normal_plot() -> Result<(), Box<dyn std::error::Error>> {
        let gp = generate_standard_normal(256, 100.0);

        let filename = "img/standard_normal.png";
        plot_values(gp, filename)
    }
}
