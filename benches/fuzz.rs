use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rect_intersect::{intersect, random_rects_detailed};

fn test(size: i32) {
    let rects = random_rects_detailed(size as _, size as _, size as _, 50);
    black_box(intersect(&rects));
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| test(black_box(100))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
