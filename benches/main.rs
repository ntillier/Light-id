use criterion::{black_box, criterion_group, criterion_main, Criterion};

use light_id::LightId;

fn bench_increment (n: usize) {
    let mut gen = LightId::new();

    for _ in 0..n {
        gen.increment();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("increment 1000000", |b| b.iter(|| bench_increment(black_box(1000000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);