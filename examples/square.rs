use numseries::series::figurate::square::square_sequence;

fn main() {
    square_sequence(10).iter().for_each(|n| println!("{}", n));
}