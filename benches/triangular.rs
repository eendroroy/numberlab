use criterion::{criterion_group, criterion_main, Criterion};
use numberlab::series::figurate::triangular::triangular_sequence;

fn generate_triangular_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_triangular_series_of_10", |b| {
        b.iter(|| triangular_sequence(10));
    });
}

fn generate_triangular_series_of_5000(c: &mut Criterion) {
    c.bench_function("generate_triangular_series_of_5000", |b| {
        b.iter(|| triangular_sequence(5000));
    });
}

criterion_group!(
    benches,
    generate_triangular_series_of_10,
    generate_triangular_series_of_5000
);

criterion_main!(benches);
