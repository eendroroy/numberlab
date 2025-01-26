use numseries::series::tribonacci::tribonacci_sequence;

fn main() {
    tribonacci_sequence(10)
        .iter()
        .for_each(|n| println!("{}", n));
}
