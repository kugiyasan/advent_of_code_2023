use criterion::{black_box, criterion_group, criterion_main, Criterion};
use d4_scratchcards::*;
use std::fs;

pub fn benchmark_part1(c: &mut Criterion) {
    let path = "input";
    let buf = fs::read_to_string(path).unwrap();
    let input: Vec<_> = buf.trim().split('\n').collect();
    c.bench_function("day 4 part 1", |b| b.iter(|| part1(black_box(&input))));
}

pub fn benchmark_part2(c: &mut Criterion) {
    let path = "input";
    let buf = fs::read_to_string(path).unwrap();
    let input: Vec<_> = buf.trim().split('\n').collect();
    c.bench_function("day 4 part 2", |b| b.iter(|| part2(black_box(&input))));
}

pub fn benchmark_part1_2(c: &mut Criterion) {
    let path = "input";
    let buf = fs::read_to_string(path).unwrap();
    let input: Vec<_> = buf.trim().split('\n').collect();
    c.bench_function("day 4 part 1 and 2", |b| {
        b.iter(|| {
            part1(black_box(&input));
            part2(black_box(&input));
        })
    });
}

criterion_group!(benches, benchmark_part1, benchmark_part2, benchmark_part1_2);
criterion_main!(benches);
