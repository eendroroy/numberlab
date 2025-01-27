use numseries::series::factorial::{factorial_sequence, nth_factorial_memoized};

fn main() {
    println!("11! : {}", nth_factorial_memoized(10));

    factorial_sequence(10)
        .iter()
        .for_each(|n| println!("{}", n));
}
