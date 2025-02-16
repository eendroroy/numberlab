use criterion::{black_box, criterion_group, criterion_main, Criterion};
use numberlab::formula::arithmetic::factorial;

fn benchmark_factorial(c: &mut Criterion) {
    c.bench_function("factorial of 12", |b| b.iter(|| factorial(black_box(12))));
}

criterion_group!(benches, benchmark_factorial);
criterion_main!(benches);
