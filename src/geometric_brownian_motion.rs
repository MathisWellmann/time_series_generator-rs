use rand_distr::{Distribution, Normal};

pub fn geometric_brownian_motion(s_0: f64, dt: f64, n: usize, drift: f64, diffusion: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0).unwrap();
    let mut v = Vec::<f64>::with_capacity(n);
    v.push(s_0);
    let drift_factor = 1.0 + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..n {
        let rv = drift_factor + diffusion_factor * dist.sample(&mut rng);
        let prod: f64 = rv * v[idx - 1];
        v.push(prod);
    }
    v
}

#[test]
fn test_geometric_brownian_motion() ->Result<(), Box<dyn std::error::Error>> {
    let vals = geometric_brownian_motion::geometric_brownian_motion(100.0, 1.5 / 365.0, 3_000, 0.15, 0.5);

    println!("vals: {:?}", vals);
    let filename = "img/geometric_brownian_motion.png";
    plt::plt(vals, filename)
}
