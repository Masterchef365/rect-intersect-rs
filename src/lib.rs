use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

pub fn intersect(rects: &[Rect]) -> Vec<(usize, usize)> {
    let mut output = vec![];
    intersect_callback(rects, &mut |a, b| output.push((a, b)));
    output
}

fn intersect_callback(rects: &[Rect], cb: &mut impl FnMut(usize, usize)) {
    #[cfg(debug_assertions)]
    for r in rects {
        debug_assert!(r.x1 <= r.x2);
        debug_assert!(r.y1 <= r.y2);
    }

    let mut v: Vec<(i32, usize)> = rects
        .iter()
        .enumerate()
        .map(|(idx, r)| [(r.x1, idx), (r.x2, idx)])
        .flatten()
        .collect();
    v.sort_by_key(|(x, _)| *x);

    detect(rects, &v, cb)
}

type SVec = smallvec::SmallVec<[usize; 4]>;
use smallvec::smallvec;

fn detect(rects: &[Rect], v: &[(i32, usize)], cb: &mut impl FnMut(usize, usize)) {
    if v.len() < 2 {
        return;
    }

    let (first_half, second_half) = v.split_at(v.len() / 2);

    let mut s11: SVec = smallvec![];
    let mut s12: SVec = smallvec![];
    let mut s21: SVec = smallvec![];
    let mut s22: SVec = smallvec![];
    let mut first_mid_touch: SVec = smallvec![];
    let mut second_mid_touch: SVec = smallvec![];

    let first = first_half.first().unwrap();
    let mid = second_half.first().unwrap();
    let end = second_half.last().unwrap();

    let mut first_y_sort: Vec<usize> = first_half.iter().map(|(_, idx)| *idx).collect();
    first_y_sort.sort_by_key(|i| rects[*i].y1);

    let mut second_y_sort: Vec<usize> = second_half.iter().map(|(_, idx)| *idx).collect();
    second_y_sort.sort_by_key(|i| rects[*i].y1);

    for i in first_y_sort {
        if rects[i].x2 == mid.0 {
            first_mid_touch.push(i);
        }

        if rects[i].x2 <= mid.0 {
            s11.push(i);
        } else {
            if rects[i].x2 >= end.0 {
                s12.push(i);
            }
        }
    }

    for i in second_y_sort {
        if rects[i].x1 == mid.0 {
            second_mid_touch.push(i);
        }

        if rects[i].x1 >= mid.0 {
            s22.push(i);
        } else {
            if rects[i].x1 <= first.0 {
                s21.push(i);
            }
        }
    }

    stab(rects, &s12, &s22, cb);
    stab(rects, &s21, &s11, cb);
    stab(rects, &s12, &s21, cb);
    stab(rects, &first_mid_touch, &second_mid_touch, cb);

    detect(rects, &first_half, cb);
    detect(rects, &second_half, cb);
}

fn stab(rects: &[Rect], a: &[usize], b: &[usize], cb: &mut impl FnMut(usize, usize)) {
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if rects[a[i]].y1 < rects[b[j]].y1 {
            let mut k = j;
            while k < b.len() && rects[b[k]].y1 <= rects[a[i]].y2 {
                cb(a[i], b[k]);
                k += 1;
            }
            i += 1;
        } else {
            let mut k = i;
            while k < a.len() && rects[a[k]].y1 <= rects[b[j]].y2 {
                cb(b[j], a[k]);
                k += 1
            }
            j += 1;
        }
    }
}

impl Rect {
    fn intersects(&self, other: &Self) -> bool {
        self.x1 <= other.x2 && other.x1 <= self.x2 && self.y1 <= other.y2 && other.y1 <= self.y2
    }
}

pub fn brute_force_intersect(rects: &[Rect]) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for i in 0..rects.len() {
        for j in i + 1..rects.len() {
            if rects[i].intersects(&rects[j]) {
                output.push((i, j));
            }
        }
    }
    output
}

#[track_caller]
pub fn to_comparable(indices: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    use std::collections::HashSet;
    let mut output = HashSet::new();
    for (a, b) in indices {
        if a == b {
            continue;
        }
        //assert_ne!(a, b);
        let mut v = [a, b];
        v.sort();
        let [l, h] = v;
        output.insert((l, h));
    }
    let mut output: Vec<(usize, usize)> = output.into_iter().collect();
    output.sort_by_key(|(_, j)| *j);
    output.sort_by_key(|(i, _)| *i);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial1() {
        let r1 = Rect {
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8,
        };

        let r2 = Rect {
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8,
        };

        assert!(r1.intersects(&r2));
        assert_eq!(
            to_comparable(intersect(&[r1, r2])),
            to_comparable(vec![(1, 0)])
        );
    }

    #[test]
    fn trivial2() {
        let r1 = Rect {
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8,
        };

        let r2 = Rect {
            x1: 50,
            x2: 60,
            y1: 2,
            y2: 8,
        };

        assert!(!r1.intersects(&r2));
        assert_eq!(to_comparable(intersect(&[r1, r2])), vec![]);
    }

    #[test]
    fn trivial3() {
        let r1 = Rect {
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8,
        };

        let r2 = Rect {
            x1: 9,
            x2: 25,
            y1: 2,
            y2: 8,
        };

        assert!(r1.intersects(&r2));
        assert_eq!(
            to_comparable(intersect(&[r1, r2])),
            to_comparable(vec![(1, 0)])
        );
    }

    #[test]
    fn random1() {
        let rects = random_rects(20, 0);
        assert_eq!(
            to_comparable(intersect(&rects)),
            to_comparable(brute_force_intersect(&rects)),
        );
    }

    #[test]
    fn random2() {
        let rects = random_rects(100, 422);
        assert_eq!(
            to_comparable(intersect(&rects)),
            to_comparable(brute_force_intersect(&rects)),
        );
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn random3() {
        let rects = random_rects(10_000, 3290);
        assert_eq!(
            to_comparable(intersect(&rects)),
            to_comparable(brute_force_intersect(&rects)),
        );
    }
}

pub fn random_rects(n: usize, seed: u64) -> Vec<Rect> {
    random_rects_detailed(n, seed, 100, 50)
}

pub fn random_rects_detailed(n: usize, seed: u64, pos_range: i32, max_size: i32) -> Vec<Rect> {
    let mut rng = SmallRng::seed_from_u64(seed);

    let range = pos_range;
    let sz = max_size;

    (0..n)
        .map(|_| {
            let x1 = rng.gen_range(-range..=range);
            let x2 = x1 + rng.gen_range(1..=sz);

            let y1 = rng.gen_range(-range..=range);
            let y2 = y1 + rng.gen_range(1..=sz);

            Rect { x1, x2, y1, y2 }
        })
        .collect()
}
