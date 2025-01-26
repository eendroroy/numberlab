use criterion::{criterion_group, criterion_main, Criterion};
use numseries::series::arithmetic::arithmetic_sequence;

fn generate_arithmetic_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_arithmetic_series_of_10", |b| {
        b.iter(|| arithmetic_sequence(1.001, 1.002,10));
    });
}

fn generate_arithmetic_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_arithmetic_series_of_500", |b| {
        b.iter(|| arithmetic_sequence(1.001, -1.002, 500));
    });
}

fn generate_arithmetic_series_of_1000(c: &mut Criterion) {
    c.bench_function("generate_arithmetic_series_of_1000", |b| {
        b.iter(|| arithmetic_sequence(7.101, -1.902, 1000));
    });
}

criterion_group!(
    benches,
    generate_arithmetic_series_of_10,
    generate_arithmetic_series_of_500,
    generate_arithmetic_series_of_1000,
);

criterion_main!(benches);
