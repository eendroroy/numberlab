use numseries::series::sylvester::sylvester_sequence;

fn main() {
    sylvester_sequence(10).iter().for_each(|n| println!("{}", n));
}
