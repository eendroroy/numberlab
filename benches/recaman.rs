use criterion::{criterion_group, criterion_main, Criterion};
use numberlab::sequence::recaman::recaman_sequence;

fn generate_recaman_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_recaman_series_of_10", |b| {
        b.iter(|| recaman_sequence(10));
    });
}

fn generate_recaman_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_recaman_series_of_500", |b| {
        b.iter(|| recaman_sequence(500));
    });
}

fn generate_recaman_series_of_1000(c: &mut Criterion) {
    c.bench_function("generate_recaman_series_of_1000", |b| {
        b.iter(|| recaman_sequence(1000));
    });
}

criterion_group!(
    benches,
    generate_recaman_series_of_10,
    generate_recaman_series_of_500,
    generate_recaman_series_of_1000,
);

criterion_main!(benches);
