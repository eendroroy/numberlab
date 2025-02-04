use numberlab::formula::permutation::permutation;

fn main() {
    println!(" 0 P 0  = {:?}", permutation(0, 0));
    println!("10 P 0  = {:?}", permutation(10, 0));
    println!("10 P 1  = {:?}", permutation(10, 1));
    println!("10 P 2  = {:?}", permutation(10, 2));
    println!("10 P 3  = {:?}", permutation(10, 3));
    println!("10 P 4  = {:?}", permutation(10, 4));
    println!("10 P 5  = {:?}", permutation(10, 5));
    println!("10 P 6  = {:?}", permutation(10, 6));
    println!("10 P 7  = {:?}", permutation(10, 7));
    println!("10 P 8  = {:?}", permutation(10, 8));
    println!("10 P 9  = {:?}", permutation(10, 9));
    println!("10 P 10 = {:?}", permutation(10, 10));
}
