# numberlab

A collection of numerical algorithms

## Installation

To use the `numberlab` library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
numberlab = "0.1.1"
```

For examples of how to use this library, refer to the documentation at [docs.rs](https://docs.rs/numberlab).
For specific examples, refer to the [`examples`](./examples) directory in the repository.

## List of algorithms

- Sequences:
    - Arithmetic Sequence
    - Factorial Sequence ([OEIS A000142](https://oeis.org/A000142))
    - Fibonacci Sequence ([OEIS A000045](https://oeis.org/A000045))
    - Geometric Sequence
    - Lucas Sequence ([OEIS A000032](https://oeis.org/A000032))
    - Recaman Sequence ([OEIS A005132](https://oeis.org/A005132))
    - Sylvester's Sequence ([OEIS A000058](https://oeis.org/A000058))
    - Tribonacci Sequence ([OEIS A000073](https://oeis.org/A000073))
    - Figurate Sequences:
        - Square Numbers ([OEIS A000290](https://oeis.org/A000290))
        - Triangular Numbers ([OEIS A000217](https://oeis.org/A000217))
        - Pentagonal Numbers ([OEIS A000326](https://oeis.org/A000326))
        - Hexagonal Numbers ([OEIS A000384](https://oeis.org/A000384))
        - Lazy Caterer's Sequence ([OEIS A000124](https://oeis.org/A000124))
- Primes:
    - Sieve:
        - Eratosthenes
        - Sundaram
