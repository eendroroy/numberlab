use numberlab::algorithm::factorize::{
    factor_pairs, factors, prime_factors, prime_factors_exponent,
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
}
