pub(crate) mod one;

mod matrix;
mod matrix_functions;
mod matrix_trait;
mod macros;

pub use matrix::Matrix;
pub use matrix_functions::{identity, lower_triangular, upper_triangular};
pub use matrix_trait::MatrixTrait;