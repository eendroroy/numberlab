use numberlab::formula::combination::combination;

fn main() {
    println!(" 0 C 0  = {:?}", combination(0, 0));
    println!("10 C 0  = {:?}", combination(10, 0));
    println!("10 C 1  = {:?}", combination(10, 1));
    println!("10 C 2  = {:?}", combination(10, 2));
    println!("10 C 3  = {:?}", combination(10, 3));
    println!("10 C 4  = {:?}", combination(10, 4));
    println!("10 C 5  = {:?}", combination(10, 5));
    println!("10 C 6  = {:?}", combination(10, 6));
    println!("10 C 7  = {:?}", combination(10, 7));
    println!("10 C 8  = {:?}", combination(10, 8));
    println!("10 C 9  = {:?}", combination(10, 9));
    println!("10 C 10 = {:?}", combination(10, 10));
}
