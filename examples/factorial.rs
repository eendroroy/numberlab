use numseries::series::factorial::factorial_sequence;

fn main() {
    factorial_sequence(10)
        .iter()
        .for_each(|n| println!("{}", n));
}
