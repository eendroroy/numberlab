use criterion::{criterion_group, criterion_main, Criterion};
use numseries::series::factorial::{factorial_sequence, nth_factorial, nth_factorial_memoized};

fn generate_5th_factorial(c: &mut Criterion) {
    c.bench_function("generate_5th_factorial", |b| {
        b.iter(|| nth_factorial(5));
    });
}

fn generate_20th_factorial(c: &mut Criterion) {
    c.bench_function("generate_20th_factorial", |b| {
        b.iter(|| nth_factorial(20));
    });
}

fn generate_5th_factorial_memoized(c: &mut Criterion) {
    c.bench_function("generate_5th_factorial_memoized", |b| {
        b.iter(|| nth_factorial_memoized(5));
    });
}

fn generate_100th_factorial_memoized(c: &mut Criterion) {
    c.bench_function("generate_100th_factorial_memoized", |b| {
        b.iter(|| nth_factorial_memoized(100));
    });
}

fn generate_factorial_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_factorial_series_of_500", |b| {
        b.iter(|| factorial_sequence(500));
    });
}

criterion_group!(
    benches,
    generate_5th_factorial,
    generate_20th_factorial,
    generate_5th_factorial_memoized,
    generate_100th_factorial_memoized,
    generate_factorial_series_of_500,
);

criterion_main!(benches);
