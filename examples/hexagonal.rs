use numseries::series::figurate::hexagonal::hexagonal_sequence;

fn main() {
    hexagonal_sequence(10).iter().for_each(|n| println!("{}", n));
}