use numberlab::sequence::arithmetic::{arithmetic_sequence, arithmetic_series, nth_arithmetic};

fn main() {
    let sequence = arithmetic_sequence::<f64>(1.5, 1.5, 10);
    println!("{:?} == {}", sequence, sequence.iter().sum::<f64>());
    let n_sequence: Vec<f64> = (1..=10).map(|n| nth_arithmetic(1.5, 1.5, n)).collect();
    println!("{:?} == {}", n_sequence, n_sequence.iter().sum::<f64>());
    println!("{}", arithmetic_series(1.5, 1.5, 10));


    println!("ODD SEQUENCE");
    let odd_sequence = arithmetic_sequence::<u32>(1, 2, 10);
    println!("{:?} == {}", odd_sequence, odd_sequence.iter().sum::<u32>());
    let n_odd_sequence: Vec<u32> = (1..=10).map(|n| nth_arithmetic(1, 2, n)).collect();
    println!(
        "{:?} == {}",
        n_odd_sequence,
        n_odd_sequence.iter().sum::<u32>()
    );
    println!("{}", arithmetic_series::<u32>(1, 2, 10));

    println!("EVEN SEQUENCE");
    let even_sequence = arithmetic_sequence::<u32>(2, 2, 10);
    println!(
        "{:?} == {}",
        even_sequence,
        even_sequence.iter().sum::<u32>()
    );
    let n_even_sequence: Vec<u32> = (1..=10).map(|n| nth_arithmetic::<u32>(2, 2, n)).collect();
    println!(
        "{:?} == {}",
        n_even_sequence,
        n_even_sequence.iter().sum::<u32>()
    );
    println!("{}", arithmetic_series::<u32>(2, 2, 10));
}
