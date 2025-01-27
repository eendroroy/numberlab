use criterion::{criterion_group, criterion_main, Criterion};
use numberlab::sequence::fibonacci::{
    fibonacci_sequence, fibonacci_series, nth_fibonacci, nth_fibonacci_memoized,
};

fn generate_5th_fibonacci(c: &mut Criterion) {
    c.bench_function("generate_5th_fibonacci", |b| {
        b.iter(|| nth_fibonacci(5));
    });
}

fn generate_20th_fibonacci(c: &mut Criterion) {
    c.bench_function("generate_20th_fibonacci", |b| {
        b.iter(|| nth_fibonacci(20));
    });
}

fn generate_5th_fibonacci_memoized(c: &mut Criterion) {
    c.bench_function("generate_5th_fibonacci_memoized", |b| {
        b.iter(|| nth_fibonacci_memoized(5));
    });
}

fn generate_100th_fibonacci_memoized(c: &mut Criterion) {
    c.bench_function("generate_100th_fibonacci_memoized", |b| {
        b.iter(|| nth_fibonacci_memoized(100));
    });
}

fn generate_fibonacci_sequence_of_500(c: &mut Criterion) {
    c.bench_function("generate_fibonacci_sequence_of_500", |b| {
        b.iter(|| fibonacci_sequence(500));
    });
}

fn generate_fibonacci_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_fibonacci_series_of_500", |b| {
        b.iter(|| fibonacci_series(500));
    });
}

criterion_group!(
    benches,
    generate_5th_fibonacci,
    generate_20th_fibonacci,
    generate_5th_fibonacci_memoized,
    generate_100th_fibonacci_memoized,
    generate_fibonacci_sequence_of_500,
    generate_fibonacci_series_of_500,
);

criterion_main!(benches);
