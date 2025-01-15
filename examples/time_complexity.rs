use std::time::Instant;

use rand::Rng;
use rect_intersect::{brute_force_intersect, intersect, random_rects, to_comparable};

fn main() {
    let mut rng = rand::thread_rng();

    println!("size,brute,algo");

    for i in 0..15 {
        let n = 2_usize.pow(i);

        let rects = random_rects(n, rng.gen());

        let algo_time = Instant::now();
        let algo = intersect(&rects);
        let algo_time = algo_time.elapsed().as_secs_f32();

        let brute_time = Instant::now();
        let brute = brute_force_intersect(&rects);
        let brute_time = brute_time.elapsed().as_secs_f32();

        println!("{},{},{}", n, brute_time, algo_time);

        assert_eq!(to_comparable(algo), to_comparable(brute),);
    }
}
