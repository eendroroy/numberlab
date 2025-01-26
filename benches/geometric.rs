use criterion::{criterion_group, criterion_main, Criterion};
use numseries::series::geometric::geometric_sequence;

fn generate_geometric_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_geometric_series_of_10", |b| {
        b.iter(|| geometric_sequence(1.001, 1.002, 10));
    });
}

fn generate_geometric_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_geometric_series_of_500", |b| {
        b.iter(|| geometric_sequence(1.001, -1.002, 500));
    });
}

fn generate_geometric_series_of_1000(c: &mut Criterion) {
    c.bench_function("generate_geometric_series_of_1000", |b| {
        b.iter(|| geometric_sequence(7.101, -1.902, 1000));
    });
}

criterion_group!(
    benches,
    generate_geometric_series_of_10,
    generate_geometric_series_of_500,
    generate_geometric_series_of_1000,
);

criterion_main!(benches);
