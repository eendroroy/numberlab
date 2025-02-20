pub(crate) mod macros;
pub(crate) mod matrix;
pub(crate) mod matrix_functions;
pub(crate) mod matrix_index;
pub(crate) mod matrix_ops;
pub(crate) mod matrix_trait;
pub(crate) mod matrix_vis;

pub use matrix::Matrix;
pub use matrix_functions::{identity, lower_triangular, transpose, upper_triangular};
pub use matrix_vis::MatrixVis;
