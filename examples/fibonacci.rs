use numseries::series::fibonacci::{fibonacci_sequence, nth_fibonacci_memoized};

fn main() {
    println!("11th Fibonacci: {}", nth_fibonacci_memoized(10));

    fibonacci_sequence(11)
        .iter()
        .for_each(|n| println!("{}", n));
}
