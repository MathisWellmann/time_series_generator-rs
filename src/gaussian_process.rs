use rand::Rng;

// rand_pct_change return a randomly generated (gaussian) number as percent change with 1% max
fn rand_gaussian_pct() -> f64 {
    let mut rng = rand::thread_rng();

    let r0: f64 = rng.gen();
    let r1: f64 = rng.gen();
    let rw: f64;
    if r0 < 0.5 {
        rw = -1f64 * r1;
    } else {
        rw = r1;
    }
    return rw.tanh() / 100.0;
}

// gaussian_prcess return gaussian sampled random values of given length
pub fn gen(length: usize, start_value: f64) -> Vec<f64> {
    let mut out= vec![0.0; length];

    out[0] = start_value;
    for i in 1..length {
        let rc = rand_gaussian_pct();
        let last = i - 1;
        out[i] = out[last] + (out[last] * rc);
    };

    return out
}

#[test]
fn test_gaussian_process() -> Result<(), Box<dyn std::error::Error>> {
    let gp = gaussian_process::gaussian_process(1000, 1000.0);
    println!("gaussian_process: {:?}", gp);

    let filename = "img/gaussian_process.png";
    plt::plt(gp, filename)
}
