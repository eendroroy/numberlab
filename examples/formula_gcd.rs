use numberlab::formula::arithmetic::gcd;

fn main() {
    println!("GCD of 12 and 15 is {}", gcd(12, 15));
    println!("GCD of 12 and 18 is {}", gcd(12, 18));
    println!("GCD of 12 and 28 is {}", gcd(12, 28));
    println!("GCD of 12 and 35 is {}", gcd(12, 35));
    println!("GCD of 12 and 36 is {}", gcd(12, 36));
    println!("GCD of 29 and 97 is {}", gcd(29, 97));
}
