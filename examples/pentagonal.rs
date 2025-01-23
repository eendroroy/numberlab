use numseries::series::figurate::pentagonal::pentagonal_sequence;

fn main() {
    pentagonal_sequence(10).iter().for_each(|n| println!("{}", n));
}