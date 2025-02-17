mod macros;
mod matrix;
mod matrix_functions;
mod matrix_index;
mod matrix_ops;
mod matrix_trait;

pub use matrix::Matrix;
pub use matrix_functions::{identity, lower_triangular, transpose, upper_triangular};
