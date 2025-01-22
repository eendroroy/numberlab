use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;
use numseries::series::fibonacci::fibonacci_sequence;

fn generate_fibonacci_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_fibonacci_series_of_10", |b| {
        b.iter(|| fibonacci_sequence(BigUint::from(0u128), 10));
    });
}

fn generate_fibonacci_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_fibonacci_series_of_500", |b| {
        b.iter(|| fibonacci_sequence(BigUint::from(0u128), 500));
    });
}

fn generate_fibonacci_series_of_1000(c: &mut Criterion) {
    c.bench_function("generate_fibonacci_series_of_1000", |b| {
        b.iter(|| fibonacci_sequence(BigUint::from(0u128), 1000));
    });
}

criterion_group!(
    benches,
    generate_fibonacci_series_of_10,
    generate_fibonacci_series_of_500,
    generate_fibonacci_series_of_1000,
);

criterion_main!(benches);
