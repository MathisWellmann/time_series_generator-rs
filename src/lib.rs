//! The timeseries-generator crate for generating various timeseries

mod geometric_brownian_motion;
mod lorenz_system;
mod mackey_glass_generator;
#[cfg(test)]
mod plot;
mod sine_wave;
mod standard_normal;
mod step_function;
mod triangle_wave;

pub use geometric_brownian_motion::generate_geometric_brownian_motion;
pub use lorenz_system::lorenz_system;
pub use mackey_glass_generator::mackey_glass_series;
pub use sine_wave::generate_sine_wave;
pub use standard_normal::generate_standard_normal;
pub use step_function::generate_step_function;
pub use triangle_wave::generate_triangle_wave;

#[cfg(test)]
pub(crate) use plot::*;
