use std::collections::VecDeque;

use rand::Rng;

/// Mackey glass series generation
///
/// # Parameters:
/// `sample_len`: the number of datapoints to generate
/// `tau`: roughly correlates to the chaotic and complex behaviour of the sequence
/// `seed`: An optional seed for the random number generator
pub fn mackey_glass_series<R: Rng>(rng: &mut R, sample_len: usize, tau: usize) -> Vec<f64> {
    let delta_t = 10;
    let mut timeseries = 1.2;
    let history_len = tau * delta_t;

    let mut history = VecDeque::with_capacity(history_len);
    for _i in 0..history_len {
        let val = 1.2 + 0.2 * (rng.random::<f64>() - 0.5);
        history.push_back(val);
    }

    let mut inp = vec![0.0; sample_len];

    for item in inp.iter_mut().take(sample_len) {
        for _ in 0..delta_t {
            let x_tau = history.pop_front().unwrap();
            history.push_back(timeseries);
            let last_hist = history[history.len() - 1];
            timeseries = last_hist
                + (0.2 * x_tau / (1.0 + x_tau.powi(10)) - 0.1 * last_hist) / delta_t as f64;
        }
        *item = timeseries;
    }
    // apply tanh nonlinearity
    inp.iter_mut().for_each(|v| *v = v.tanh());

    inp
}

#[cfg(test)]
mod tests {
    use rand::{rngs::SmallRng, SeedableRng};

    use crate::plot::plot_2d;

    use super::*;

    #[test]
    fn plot_mackey_glass() {
        let mut rng = SmallRng::seed_from_u64(0);
        let series = mackey_glass_series(&mut rng, 1000, 30);

        let filename = "./img/mackey_glass.png";
        plot_2d(series, filename).unwrap();
    }
}
