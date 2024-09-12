use std::ops::{Deref, DerefMut};

use actix_web::{dev::ResourcePath, web::BufMut};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

struct Media {
    path: String,
}

impl Media {
    pub fn default() -> String {
        "Default".to_string()
    }
}

fn remove_first(array: Vec<Media>) -> String {
    return array
        .into_iter()
        .next()
        .map_or_else(|| Media::default(), |c| c.path);
}

fn remove_second(array: Vec<Media>) -> String {
    return array
        .first()
        .map_or_else(|| Media::default(), |c| c.path.clone());
}

fn remove_third(mut array: Vec<Media>) -> String {
    if array.is_empty() {
        Media::default()
    } else {
        array.swap_remove(0).path
    }
}

fn strange() {
    let el = [2];
    let [v_] = el;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("remove first", |b| {
        b.iter(|| remove_first(black_box(Vec::with_capacity(20))))
    });
}

fn bench_removes(c: &mut Criterion) {
    let mut group = c.benchmark_group("remove");
    for i in [0, 10, 10000000].iter() {
        group.bench_with_input(BenchmarkId::new("iterator", i), i, |b, i| {
            b.iter(|| remove_first(Vec::with_capacity(*i)))
        });
        group.bench_with_input(BenchmarkId::new("array", i), i, |b, i| {
            b.iter(|| remove_second(Vec::with_capacity(*i)))
        });
        group.bench_with_input(BenchmarkId::new("swap", i), i, |b, i| {
            b.iter(|| remove_third(Vec::with_capacity(*i)))
        });
    }
}

criterion_group!(benches, bench_removes);
criterion_main!(benches);
