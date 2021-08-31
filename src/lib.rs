#![deny(missing_docs, rustdoc::missing_crate_level_docs)]

//! The timeseries-generator crate for generating randomly sampled timeseries

mod geometric_brownian_motion;
#[cfg(test)]
mod plot;
mod sine_wave;
mod standard_normal;
mod step_function;
mod triangle_wave;

pub use geometric_brownian_motion::generate_geometric_brownian_motion;
pub use sine_wave::generate_sine_wave;
pub use standard_normal::generate_standard_normal;
pub use step_function::generate_step_function;
pub use triangle_wave::generate_triangle_wave;

#[cfg(test)]
pub(crate) use plot::plot_values;
