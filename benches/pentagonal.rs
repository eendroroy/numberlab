use criterion::{criterion_group, criterion_main, Criterion};
use numseries::series::figurate::pentagonal::pentagonal_sequence;

fn generate_pentagonal_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_pentagonal_series_of_10", |b| {
        b.iter(|| pentagonal_sequence(10));
    });
}

fn generate_pentagonal_series_of_5000(c: &mut Criterion) {
    c.bench_function("generate_pentagonal_series_of_5000", |b| {
        b.iter(|| pentagonal_sequence(5000));
    });
}

criterion_group!(
    benches,
    generate_pentagonal_series_of_10,
    generate_pentagonal_series_of_5000
);

criterion_main!(benches);
