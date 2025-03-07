use num_traits::Float;
use rand::Rng;
use rand_distr::{Distribution, Normal, StandardNormal};

/// Generate geometric brownian motion
pub fn generate_geometric_brownian_motion<R, T>(
    rng: &mut R,
    s_0: T,
    dt: T,
    length: usize,
    drift: T,
    diffusion: T,
) -> Vec<T>
where
    R: Rng,
    T: Float,
    StandardNormal: rand_distr::Distribution<T>,
{
    let dist = Normal::<T>::new(T::zero(), T::one()).expect("Is valid distribution");
    let mut v = Vec::<T>::with_capacity(length);
    v.push(s_0);
    let drift_factor = T::one() + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..length {
        let rv = drift_factor + diffusion_factor * dist.sample(rng);
        let prod: T = rv * v[idx - 1];
        v.push(prod);
    }
    v
}

#[cfg(test)]
mod tests {
    use rand::{rngs::SmallRng, SeedableRng};

    use super::*;
    use crate::plot::plot_2d;

    #[test]
    fn geometric_brownian_motion_plot() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = SmallRng::seed_from_u64(0);
        let vals = generate_geometric_brownian_motion(&mut rng, 100.0, 1.5 / 365.0, 256, 0.15, 0.5);

        let filename = "img/geometric_brownian_motion.png";
        plot_2d(vals, filename)
    }
}
