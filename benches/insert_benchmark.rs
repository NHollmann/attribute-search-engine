use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration, Throughput,
};
use std::{hint::black_box, time::Duration};

mod indices;
use indices::*;

fn insert_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    group.measurement_time(Duration::from_secs(10));
    group
        .plot_config(PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic));

    for &size in [1000, 10000, 100000].iter() {
        let mut input = Vec::with_capacity(size);
        for i in 0..size {
            input.push(format!("{:06}", i % (size / 50)).chars().rev().collect());
        }

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new("SearchIndexHashMap", size),
            &input,
            |b, input| {
                b.iter(|| create_index_hashmap(black_box(input)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("SearchIndexPrefixTree", size),
            &input,
            |b, input| {
                b.iter(|| create_index_prefix_tree(black_box(input)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("SearchIndexBTreeRange", size),
            &input,
            |b, input| {
                b.iter(|| create_index_btree_range(black_box(input)));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, insert_bench);
criterion_main!(benches);
