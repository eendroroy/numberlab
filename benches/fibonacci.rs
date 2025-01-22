use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;
use numseries::series::fibonacci::fibonacci_sequence;

fn generate_fibonacci_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_fibonacci_series_of_10", |b| {
        b.iter(|| fibonacci_sequence(BigUint::from(0u128), 10));
    });
}

fn generate_fibonacci_series_of_5000(c: &mut Criterion) {
    c.bench_function("generate_fibonacci_series_of_5000", |b| {
        b.iter(|| fibonacci_sequence(BigUint::from(0u128), 10));
    });
}

criterion_group!(
    benches,
    generate_fibonacci_series_of_10,
    generate_fibonacci_series_of_5000
);

criterion_main!(benches);
