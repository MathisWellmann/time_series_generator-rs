mod gaussian_process;
mod sine_wave;
mod step_function;

fn main() {
    let length: usize = 1_000;
    let gp = gaussian_process::gaussian_process(length, 1000.0);
    println!("gaussian_process: {:?}", gp);
    let sw = sine_wave::sine_wave(length);
    println!("sine_wave: {:?}", sw);
    let sf = step_function::step_function(length, length / 2, length / 10);
    println!("step_function: {:?}", sf);
}

mod plt;

#[test]
fn test_step_function() -> Result<(), Box<dyn std::error::Error>> {
    let sf = step_function::step_function(1000, 500, 100);
    println!("sf: {:?}", sf);

    let filename = "img/step_function.png";
    plt::plt(sf, filename)
}

#[test]
fn test_gaussian_process() -> Result<(), Box<dyn std::error::Error>> {
    let gp = gaussian_process::gaussian_process(1000, 1000.0);
    println!("gaussian_process: {:?}", gp);

    let filename = "img/gaussian_process.png";
    plt::plt(gp, filename)
}

#[test]
fn test_sine_wave() -> Result<(), Box<dyn std::error::Error>> {
    let sw = sine_wave::sine_wave(1024);
    println!("sine_wave: {:?}", sw);

    let filename = "img/sine_wave.png";
    plt::plt(sw, filename)
}