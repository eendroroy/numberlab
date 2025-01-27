use criterion::{criterion_group, criterion_main, Criterion};
use numberlab::series::figurate::square::square_sequence;

fn generate_square_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_square_series_of_10", |b| {
        b.iter(|| square_sequence(10));
    });
}

fn generate_square_series_of_5000(c: &mut Criterion) {
    c.bench_function("generate_square_series_of_5000", |b| {
        b.iter(|| square_sequence(5000));
    });
}

criterion_group!(
    benches,
    generate_square_series_of_10,
    generate_square_series_of_5000
);

criterion_main!(benches);
