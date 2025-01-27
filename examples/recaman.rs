use numberlab::series::recaman::{nth_recaman_memoized, recaman_sequence};

fn main() {
    let mut memoizer = vec![];
    println!("11th Recaman: {}", nth_recaman_memoized(&mut memoizer, 10));

    let n = 100;
    recaman_sequence(n).iter().for_each(
        |recaman| println!("{}", recaman)
    );
}
