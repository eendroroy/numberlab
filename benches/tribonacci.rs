use criterion::{criterion_group, criterion_main, Criterion};
use numberlab::sequence::tribonacci::{nth_tribonacci, nth_tribonacci_memoized, tribonacci_sequence};

fn generate_5th_tribonacci(c: &mut Criterion) {
    c.bench_function("generate_5th_tribonacci", |b| {
        b.iter(|| nth_tribonacci(5));
    });
}

fn generate_20th_tribonacci(c: &mut Criterion) {
    c.bench_function("generate_20th_tribonacci", |b| {
        b.iter(|| nth_tribonacci(20));
    });
}

fn generate_5th_tribonacci_memoized(c: &mut Criterion) {
    c.bench_function("generate_5th_tribonacci_memoized", |b| {
        b.iter(|| nth_tribonacci_memoized(5));
    });
}

fn generate_100th_tribonacci_memoized(c: &mut Criterion) {
    c.bench_function("generate_100th_tribonacci_memoized", |b| {
        b.iter(|| nth_tribonacci_memoized(100));
    });
}

fn generate_tribonacci_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_tribonacci_series_of_500", |b| {
        b.iter(|| tribonacci_sequence(500));
    });
}

criterion_group!(
    benches,
    generate_5th_tribonacci,
    generate_20th_tribonacci,
    generate_5th_tribonacci_memoized,
    generate_100th_tribonacci_memoized,
    generate_tribonacci_series_of_500,
);

criterion_main!(benches);
