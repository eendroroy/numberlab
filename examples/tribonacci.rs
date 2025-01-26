use numseries::series::tribonacci::{nth_tribonacci_memoized, tribonacci_sequence};

fn main() {
    let mut memoizer = vec![];
    println!("11th Fibonacci: {}", nth_tribonacci_memoized(10, &mut memoizer));

    tribonacci_sequence(11)
        .iter()
        .for_each(|n| println!("{}", n));
}
