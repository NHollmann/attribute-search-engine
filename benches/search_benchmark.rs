use attribute_search_engine::{SearchEngine, SearchIndexHashMap};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration};
use std::{hint::black_box, time::Duration};

fn create_engine(n: usize) {
    let mut index_a = SearchIndexHashMap::<_, String>::new();
    let mut index_b = SearchIndexHashMap::<_, String>::new();
    let mut index_c = SearchIndexHashMap::<_, String>::new();
    let mut index_d = SearchIndexHashMap::<_, String>::new();

    for i in 0..n {
        index_a.insert(i, format!("{}", i % 10));
        if i % 2 == 0 {
            index_b.insert(i, format!("{}", i % 3));
        }
        if i % 5 == 0 {
            index_c.insert(i, format!("{}", i % 25));
        }
        if i % 5 == 2 {
            index_b.insert(i, format!("{}", i % 13));
        }
        if i % 7 == 0 {
            index_d.insert(i, format!("{}", i % 5));
        }
    }
    let mut engine = SearchEngine::<usize>::new();
    engine.add_index("a", index_a);
    engine.add_index("b", index_b);
    engine.add_index("c", index_c);
    engine.add_index("d", index_d);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut create_group = c.benchmark_group("create engine");
    create_group
        .sample_size(150)
        .measurement_time(Duration::from_secs(15));
    create_group
        .plot_config(PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic));
    for size in [1, 10, 100, 1000, 10000, 100000].iter() {
        create_group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| create_engine(black_box(size)));
        });
    }
    create_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
