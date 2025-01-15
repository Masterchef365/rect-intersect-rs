use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rect_intersect::{brute_force_intersect, intersect, random_rects_detailed};


pub fn criterion_benchmark(c: &mut Criterion) {
    for i in (10..90).step_by(20) {
        let rects = random_rects_detailed((i*i) as _, i as _, (i * 15) as _, 50);
        c.bench_function(&format!("random {}", i*i), |b| b.iter(|| {
            black_box(intersect(black_box(&rects)));
        }));
        c.bench_function(&format!("brute {}", i*i), |b| b.iter(|| {
            black_box(brute_force_intersect(black_box(&rects)));
        }));

    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
