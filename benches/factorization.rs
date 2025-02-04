use criterion::{black_box, criterion_group, criterion_main, Criterion};
use numberlab::algorithm::factorize::factors;

fn benchmark_factors(c: &mut Criterion) {
    c.bench_function("factors of 3991680", |b| {
        b.iter(|| factors(black_box(3991680)))
    });
}

fn benchmark_prime_factors(c: &mut Criterion) {
    c.bench_function("prime_factors of 3991680", |b| {
        b.iter(|| numberlab::algorithm::factorize::prime_factors(black_box(3991680)))
    });
}

criterion_group!(benches, benchmark_factors, benchmark_prime_factors);
criterion_main!(benches);
