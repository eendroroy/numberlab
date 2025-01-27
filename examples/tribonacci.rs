use numberlab::sequence::tribonacci::{nth_tribonacci_memoized, tribonacci_sequence};

fn main() {
    println!("11th tribonacci: {}", nth_tribonacci_memoized(10));

    tribonacci_sequence(11)
        .iter()
        .for_each(|n| println!("{}", n));
}
