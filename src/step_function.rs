/// Generates a vector of given length with step function using mid_point and width parameter
pub fn generate_step_function(length: usize, mid_point: usize, width: usize) -> Vec<f64> {
    let mut out = vec![0.0; length];

    for (i, val) in out.iter_mut().enumerate() {
        if i < mid_point - width || i > mid_point + width {
            *val = 0.0;
        } else {
            *val = 1.0;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot_2d;

    #[test]
    fn step_function_plot() -> Result<(), Box<dyn std::error::Error>> {
        let sf = generate_step_function(1000, 500, 100);

        let filename = "img/step_function.png";
        plot_2d(sf, filename)
    }
}
