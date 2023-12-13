#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use light_id::LightId;

fn bench_increment (mut gen: LightId, n: usize) {
    for _ in 0..n {
        gen.increment();
    }
}

fn bench_increment_by (mut gen: LightId, n: usize) {
    gen.increment_by(n);
}

fn bench_last (mut gen: LightId, last: &str) {
    gen.last(last);
}

fn bench_current (mut gen: LightId) {
    gen.current();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut gen = LightId::new();

    c.bench_function("increment 1000000", |b| b.iter(|| bench_increment(black_box(gen.clone()), black_box(1000000))));
    c.bench_function("increment by 1000000", |b| b.iter(|| bench_increment_by(black_box(gen.clone()), black_box(1000000))));

    c.bench_function("last aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", |b| b.iter(|| bench_last(black_box(gen.clone()), black_box("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"))));
    c.bench_function("last cbabcacbacbcacbcabcbacbacbacacacbacbcbc", |b| b.iter(|| bench_last(black_box(gen.clone()), black_box("cbabcacbacbcacbcabcbacbacbacacacbacbcbc"))));
    c.bench_function("last z0sq80snqucnoq8c79e4jk2nhcfgzpajdk3j48fyvz893b2x", |b| b.iter(|| bench_last(black_box(gen.clone()), black_box("z0sq80snqucnoq8c79e4jk2nhcfgzpajdk3j48fyvz893b2x"))));

    gen.skip(10000);
    c.bench_function("current - 10000", |b| b.iter(|| bench_current(black_box(gen.clone()))));
    
    gen.skip(1000000);
    c.bench_function("current - 1000000", |b| b.iter(|| bench_current(black_box(gen.clone()))));
    
    gen.skip(100000000);
    c.bench_function("current - 100000000", |b| b.iter(|| bench_current(black_box(gen.clone()))));
    
    gen.skip(100000000000);
    c.bench_function("current - 100000000000", |b| b.iter(|| bench_current(black_box(gen.clone()))));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);