use criterion::{criterion_group, criterion_main, Criterion};
use numseries::series::figurate::triangle::triangle_sequence;

fn generate_triangle_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_triangle_series_of_10", |b| {
        b.iter(|| triangle_sequence(10));
    });
}

fn generate_triangle_series_of_5000(c: &mut Criterion) {
    c.bench_function("generate_triangle_series_of_5000", |b| {
        b.iter(|| triangle_sequence(5000));
    });
}

criterion_group!(
    benches,
    generate_triangle_series_of_10,
    generate_triangle_series_of_5000
);

criterion_main!(benches);
