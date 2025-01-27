use numberlab::series::figurate::triangular::triangular_sequence;

fn main() {
    triangular_sequence(10)
        .iter()
        .for_each(|n| println!("{}", n));
}
