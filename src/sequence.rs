pub mod factorial;

/// Arithmetic sequence
pub mod arithmetic;
/// Fibonacci sequence [OEIS A000045](https://oeis.org/A000045)
pub mod fibonacci;
/// Geometric sequence
pub mod geometric;
/// Lucas sequence [OEIS A000032](https://oeis.org/A000032)
pub mod lucas;
/// Recaman sequence [OEIS A005132](https://oeis.org/A005132)
pub mod recaman;
/// Sylvester's sequence [OEIS A000058](https://oeis.org/A000058)
pub mod sylvester;
/// Tribonacci sequence [OEIS A000073](https://oeis.org/A000073)
pub mod tribonacci;

pub use factorial::factorial_sequence;
