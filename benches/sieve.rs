use criterion::{criterion_group, criterion_main, Criterion};
use numseries::series::prime::sieve::sieve_prime_sequence;

fn generate_sieve_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_sieve_series_of_10", |b| {
        b.iter(|| sieve_prime_sequence(10));
    });
}

fn generate_sieve_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_sieve_series_of_500", |b| {
        b.iter(|| sieve_prime_sequence(500));
    });
}

fn generate_sieve_series_of_1000(c: &mut Criterion) {
    c.bench_function("generate_sieve_series_of_1000", |b| {
        b.iter(|| sieve_prime_sequence(1000));
    });
}

criterion_group!(
    benches,
    generate_sieve_series_of_10,
    generate_sieve_series_of_500,
    generate_sieve_series_of_1000,
);

criterion_main!(benches);
