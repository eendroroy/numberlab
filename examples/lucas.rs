use numseries::series::lucas::lucas_sequence;

fn main() {
    lucas_sequence(10).iter().for_each(|n| println!("{}", n));
}
