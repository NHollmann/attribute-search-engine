use attribute_search_engine::{Query, SearchIndex};
use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration, Throughput,
};
use std::{hint::black_box, time::Duration};

mod indices;
use indices::*;

fn search_index(index: &impl SearchIndex<usize>, queries: &[Query]) {
    for q in queries {
        index.search(q).expect("no error");
    }
}

fn search_exact_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("search-exact");
    group.measurement_time(Duration::from_secs(10));
    group
        .plot_config(PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic));

    let index_size = 1000000;
    let mut input = Vec::with_capacity(index_size);
    for i in 0..index_size {
        input.push(
            format!("{:06}", i % (index_size / 100))
                .chars()
                .rev()
                .collect(),
        );
    }
    let index_hashmap = create_index_hashmap(&input);
    let index_prefix_tree = create_index_prefix_tree(&input);
    let index_btree_range = create_index_btree_range(&input);

    for &size in [100, 1000, 10000].iter() {
        let mut input = Vec::with_capacity(size);
        for i in 0..size {
            input.push(Query::Exact(
                "".into(),
                format!("{:06}", i % (size / 50)).chars().rev().collect(),
            ));
        }

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new("SearchIndexHashMap", size),
            &input,
            |b, input| {
                b.iter(|| search_index(&index_hashmap, black_box(input)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("SearchIndexPrefixTree", size),
            &input,
            |b, input| {
                b.iter(|| search_index(&index_prefix_tree, black_box(input)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("SearchIndexBTreeRange", size),
            &input,
            |b, input| {
                b.iter(|| search_index(&index_btree_range, black_box(input)));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, search_exact_bench);
criterion_main!(benches);
