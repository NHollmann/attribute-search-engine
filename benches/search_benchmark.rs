use attribute_search_engine::{AttributeKind, AttributeSchema, SearchEngine};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration};
use std::{hint::black_box, time::Duration};

fn create_engine(n: usize) {
    let mut schema = AttributeSchema::new();
    schema.register_attribute("a", AttributeKind::ExactMatch);
    schema.register_attribute("b", AttributeKind::ExactMatch);
    schema.register_attribute("c", AttributeKind::ExactMatch);
    schema.register_attribute("d", AttributeKind::ExactMatch);

    let mut engine = SearchEngine::new(&schema);
    for i in 0..n {
        engine.insert(i, "a", &format!("{}", i % 10)).unwrap();
        if i % 2 == 0 {
            engine.insert(i, "b", &format!("{}", i % 3)).unwrap();
        }
        if i % 5 == 0 {
            engine.insert(i, "c", &format!("{}", i % 25)).unwrap();
        }
        if i % 5 == 2 {
            engine.insert(i, "b", &format!("{}", i % 13)).unwrap();
        }
        if i % 7 == 0 {
            engine.insert(i, "d", &format!("{}", i % 5)).unwrap();
        }
    }
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
