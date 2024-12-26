use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use attribute_search_engine::add;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add", |b| b.iter(|| add(black_box(20), black_box(30))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
