use criterion::{criterion_group, criterion_main, Criterion};
use numberlab::sequence::figurate::lazy_caterer::lazy_caterer_sequence;

fn generate_lazy_caterer_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_lazy_caterer_series_of_10", |b| {
        b.iter(|| lazy_caterer_sequence(10));
    });
}

fn generate_lazy_caterer_series_of_5000(c: &mut Criterion) {
    c.bench_function("generate_lazy_caterer_series_of_5000", |b| {
        b.iter(|| lazy_caterer_sequence(5000));
    });
}

criterion_group!(
    benches,
    generate_lazy_caterer_series_of_10,
    generate_lazy_caterer_series_of_5000
);

criterion_main!(benches);
