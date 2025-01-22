use numseries::series::figurate::triangle::triangle_sequence;

fn main() {
    triangle_sequence(10).iter().for_each(|n| println!("{}", n));
}