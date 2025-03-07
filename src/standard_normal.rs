use num_traits::Float;
use rand::prelude::*;
use rand_distr::StandardNormal;

/// generate a new randomly sampled timeseries of given length
/// using a standard normal distribution scaled down by 100
/// by multiplying a random (+/-) value with the previous value
pub fn generate_standard_normal<R, T>(rng: &mut R, length: usize, start_value: T) -> Vec<T>
where
    T: Float,
    R: Rng,
    StandardNormal: rand_distr::Distribution<T>,
{
    let mut out: Vec<T> = vec![T::zero(); length];

    out.push(start_value);
    let mut last_val = start_value;
    for v in out.iter_mut() {
        let r: T = rng.sample(StandardNormal);
        *v = last_val + (last_val * (r / T::from(100.0).expect("can convert")));
        last_val = *v;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot_2d;

    #[test]
    fn generate_standard_normal_plot() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = SmallRng::seed_from_u64(0);
        let gp = generate_standard_normal(&mut rng, 256, 100.0);

        let filename = "img/standard_normal.png";
        plot_2d(gp, filename)
    }
}
