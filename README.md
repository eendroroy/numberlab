# numberlab

A collection of numerical algorithms

## Overview

`numberlab` is a Rust library that provides a variety of numerical algorithms, including matrix operations, prime number
sieves, and various sequences. It is designed to be easy to use and efficient.

## Installation

To use the `numberlab` library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
numberlab = "0.1.8"
```

For examples of how to use this library, refer to the documentation at [docs.rs](https://docs.rs/numberlab).
For specific examples, refer to the [`examples`](./examples) directory in the repository.

## List of algorithms

- Algorithms:
    - Factorization
    - Graph
        - BFS
        - DFS
        - Dijkstra
        - A*
    - Matrix
        - BFS
        - DFS
        - Dijkstra
- Coordinates:
    - Cartesian
    - Polar
- Figurate Sequences:
    - Hexagonal Numbers ([OEIS A000384](https://oeis.org/A000384))
    - Lazy Caterer's Sequence ([OEIS A000124](https://oeis.org/A000124))
    - Pentagonal Numbers ([OEIS A000326](https://oeis.org/A000326))
    - Square Numbers ([OEIS A000290](https://oeis.org/A000290))
    - Star Numbers ([OEIS A003154](https://oeis.org/A003154))
    - Stella Octangula Numbers ([OEIS A007588](https://oeis.org/A007588))
    - Triangular Numbers ([OEIS A000217](https://oeis.org/A000217))
- Formula
    - Arithmetic
        - Combination
        - Factorial
        - GCD
        - LCM
        - Permutation
- Patterns:
    - Pascal's Triangle ([OEIS A007318](https://oeis.org/A007318))
- Primes:
    - Sieve:
        - Eratosthenes
        - Sundaram
- Sequences:
    - Arithmetic Sequence
    - Factorial Sequence ([OEIS A000142](https://oeis.org/A000142))
    - Fibonacci Sequence ([OEIS A000045](https://oeis.org/A000045))
    - Geometric Sequence
    - Lucas Sequence ([OEIS A000032](https://oeis.org/A000032))
    - Recaman Sequence ([OEIS A005132](https://oeis.org/A005132))
    - Sylvester's Sequence ([OEIS A000058](https://oeis.org/A000058))
    - Tribonacci Sequence ([OEIS A000073](https://oeis.org/A000073))
- Structures:
    - Matrix
    - Graph

## License

This project is licensed under the [GNU AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.html). See
the [LICENSE](./LICENSE) file for more details.
