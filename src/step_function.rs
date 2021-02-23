/// Generates a vector of given length with step function using mid_point and width parameter
pub fn generate_step_function(length: usize, mid_point: usize, width: usize) -> Vec<f64> {
    let mut out = vec![0.0; length];

    for i in 0..length {
        if i < mid_point - width || i > mid_point + width {
            out[i] = 0.0;
            continue;
        }
        out[i] = 1.0;
    }
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot_values;

    #[test]
    fn step_function_plot() -> Result<(), Box<dyn std::error::Error>> {
        let sf = generate_step_function(1000, 500, 100);

        let filename = "img/step_function.png";
        plot_values(sf, filename)
    }
}
