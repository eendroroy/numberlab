use numberlab::algorithm::factorize::{
    factor_pairs, factors, gcd, lcm, prime_factors, prime_factors_exponent,
};

fn main() {
    let n = 28;
    println!(
        "factors                {} => {}",
        n,
        factors(n)
            .iter()
            .map(usize::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!(
        "factor_pairs           {} == {}",
        n,
        factor_pairs(n)
            .iter()
            .map(|(a, b)| format!("{} * {}", a, b))
            .collect::<Vec<String>>()
            .join(" == ")
    );
    println!(
        "prime_factors          {}  = {}",
        n,
        prime_factors(n)
            .iter()
            .map(usize::to_string)
            .collect::<Vec<String>>()
            .join(" * ")
    );
    println!(
        "prime_factors_exponent {}  = {}",
        n,
        prime_factors_exponent(n)
            .iter()
            .map(|(k, v)| format!("{}^{}", k, v))
            .collect::<Vec<String>>()
            .join(" * ")
    );

    println!();

    println!("LCM of 12 and 15 is {}", lcm(12, 15));
    println!("LCM of 12 and 18 is {}", lcm(12, 18));
    println!("LCM of 12 and 28 is {}", lcm(12, 28));
    println!("LCM of 12 and 35 is {}", lcm(12, 35));
    println!("LCM of 12 and 36 is {}", lcm(12, 36));
    println!("LCM of 29 and 97 is {}", lcm(29, 97));

    println!();

    println!("GCD of 12 and 15 is {}", gcd(12, 15));
    println!("GCD of 12 and 18 is {}", gcd(12, 18));
    println!("GCD of 12 and 28 is {}", gcd(12, 28));
    println!("GCD of 12 and 35 is {}", gcd(12, 35));
    println!("GCD of 12 and 36 is {}", gcd(12, 36));
    println!("GCD of 29 and 97 is {}", gcd(29, 97));
}
