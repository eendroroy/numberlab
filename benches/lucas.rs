use criterion::{criterion_group, criterion_main, Criterion};
use numseries::series::lucas::lucas_sequence;

fn generate_lucas_series_of_10(c: &mut Criterion) {
    c.bench_function("generate_lucas_series_of_10", |b| {
        b.iter(|| lucas_sequence(10));
    });
}

fn generate_lucas_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_lucas_series_of_500", |b| {
        b.iter(|| lucas_sequence(500));
    });
}

fn generate_lucas_series_of_1000(c: &mut Criterion) {
    c.bench_function("generate_lucas_series_of_1000", |b| {
        b.iter(|| lucas_sequence(1000));
    });
}

criterion_group!(
    benches,
    generate_lucas_series_of_10,
    generate_lucas_series_of_500,
    generate_lucas_series_of_1000,
);

criterion_main!(benches);
