use criterion::{black_box, criterion_group, criterion_main, Criterion};
use d6_wait_boat::*;
use std::fs;

pub fn benchmark_iter(c: &mut Criterion) {
    let path = "input";
    let buf = fs::read_to_string(path).unwrap();
    c.bench_function("day 6 iter", |b| {
        b.iter(|| iterator_version(black_box(&buf)))
    });
}

pub fn benchmark_forloop(c: &mut Criterion) {
    let path = "input";
    let buf = fs::read_to_string(path).unwrap();
    c.bench_function("day 6 forloop", |b| {
        b.iter(|| forloop_version(black_box(&buf)))
    });
}

criterion_group!(benches, benchmark_iter, benchmark_forloop);
criterion_main!(benches);
