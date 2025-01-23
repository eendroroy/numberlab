use numseries::series::figurate::lazy_caterer::lazy_caterer_sequence;

fn main() {
    lazy_caterer_sequence(10).iter().for_each(|n| println!("{}", n));
}