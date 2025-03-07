use rand::Rng;
use rand_distr::{Distribution, Normal};

/// Generate geometric brownian motion
pub fn generate_geometric_brownian_motion<R: Rng>(
    rng: &mut R,
    s_0: f64,
    dt: f64,
    length: usize,
    drift: f64,
    diffusion: f64,
) -> Vec<f64> {
    let dist = Normal::new(0.0, 1.0).expect("Is valid distribution");
    let mut v = Vec::<f64>::with_capacity(length);
    v.push(s_0);
    let drift_factor = 1.0 + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..length {
        let rv = drift_factor + diffusion_factor * dist.sample(rng);
        let prod: f64 = rv * v[idx - 1];
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
