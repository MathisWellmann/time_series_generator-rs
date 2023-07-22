use num_traits::Float;

/// Generate a lorenz system in 3D space.
///
/// # Arguments:
/// `sigma`: Lorenz uses 10.0
/// `beta`: Lorenz uses 8 / 3
/// `rho`: Lorenz uses 28
/// `sampling_len`: How many datapoints to return
///
/// # Returns:
/// Three vectors each describing a single variable of the state space.
/// All vectors have the same length.
///
pub fn lorenz_system<T: Float>(
    sigma: T,
    beta: T,
    rho: T,
    sampling_len: usize,
    dt: T,
) -> Vec<(T, T, T)> {
    // starting conditions
    let mut x = T::one();
    let mut y = T::zero();
    let mut z = T::zero();

    let mut out = vec![(T::zero(), T::zero(), T::zero()); sampling_len];

    for i in 0..sampling_len {
        out[i] = (x, y, z);

        x = x + (sigma * (y - x)) * dt;
        y = y + (x * (rho - z) - y) * dt;
        z = z + (x * y - beta * z) * dt;
    }

    out
}

#[cfg(test)]
mod test {
    use crate::plot::{plot_3d, Series3D};

    use super::*;

    #[test]
    fn plot_lorenz_system_3d() {
        let series = lorenz_system(10.0, 2.667, 28.0, 5000, 0.01);
        println!("series: {series:?}");

        plot_3d(&Series3D(series), "img/lorenz_system_3d.png", (1024, 1024)).unwrap();
    }
}
