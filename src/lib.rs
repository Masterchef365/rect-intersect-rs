use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub id: usize,
}

pub fn intersect(rects: &[Rect]) -> Vec<(usize, usize)> {
    let mut output = vec![];
    intersect_callback(rects, &mut |a, b| output.push(dbg!(a, b)));
    output
}

fn intersect_callback(rects: &[Rect], cb: &mut impl FnMut(usize, usize)) {
    #[cfg(debug_assertions)]
    for r in rects {
        debug_assert!(r.x1 <= r.x2);
        debug_assert!(r.y1 <= r.y2);
    }

    let mut v: Vec<(i32, Rect)> = rects
        .iter()
        .map(|r| [(r.x1, *r), (r.x2, *r)])
        .flatten()
        .collect();
    v.sort_by_key(|(x, _)| *x);

    detect(&v, cb)
}

fn detect(v: &[(i32, Rect)], cb: &mut impl FnMut(usize, usize)) {
    eprintln!();
    if v.len() < 2 {
        return;
    }

    let (first_half, second_half) = v.split_at(v.len() / 2);

    let mut s11 = vec![];
    let mut s12 = vec![];
    let mut s21 = vec![];
    let mut s22 = vec![];

    let first = first_half.first().unwrap().0;
    let mid = second_half.first().unwrap().0;
    let end = second_half.last().unwrap().0;

    let mut first_y_sort: Vec<Rect> = first_half.iter().map(|(_, r)| *r).collect();
    first_y_sort.sort_by_key(|r| r.y1);

    let mut second_y_sort: Vec<Rect> = second_half.iter().map(|(_, r)| *r).collect();
    second_y_sort.sort_by_key(|r| r.y1);

    dbg!((first, mid, end, first_y_sort.len(), second_y_sort.len()));

    for rect in first_y_sort {
        if rect.x2 < mid {
            s11.push(rect);
        } else {
            if rect.x2 >= end {
                s12.push(rect);
            }
        }
    }

    for rect in second_y_sort {
        if rect.x1 > mid {
            s22.push(rect);
        } else {
            if rect.x1 < first {
                s21.push(rect);
            }
        }
    }

    eprintln!("S12 S22");
    stab(&s12, &s22, cb);
    eprintln!("S21 S11");
    stab(&s21, &s11, cb);
    eprintln!("S12 S21");
    stab(&s12, &s21, cb);

    //eprintln!("S21 S22");
    //stab(&s21, &s22, cb);

    detect(&first_half, cb);
    detect(&second_half, cb);
}

fn stab(a: &[Rect], b: &[Rect], cb: &mut impl FnMut(usize, usize)) {
    println!("{:?} {:?}", a.first(), b.last());
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i].y1 <= b[j].y1 {
            let mut k = j;
            while k < b.len() && b[k].y1 <= a[i].y2 {
                cb(a[i].id, b[k].id);
                k += 1;
            }
            i += 1;
        } else {
            let mut k = i;
            while k < a.len() && a[k].y1 <= b[j].y2 {
                cb(b[j].id, a[k].id);
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
                output.push((rects[i].id, rects[j].id));
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
        assert_ne!(a, b);
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
            id: 0,
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8,
        };

        let r2 = Rect {
            id: 1,
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
            id: 0,
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8,
        };

        let r2 = Rect {
            id: 1,
            x1: 50,
            x2: 60,
            y1: 2,
            y2: 8,
        };

        assert!(!r1.intersects(&r2));
        assert_eq!(intersect(&[r1, r2]), vec![]);
    }

    #[test]
    fn trivial3() {
        let r1 = Rect {
            id: 0,
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8,
        };

        let r2 = Rect {
            id: 1,
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
}

pub fn random_rects(n: usize, seed: u64) -> Vec<Rect> {
    let mut rng = SmallRng::seed_from_u64(seed);

    let range = 100;
    let sz = 50;

    (0..n)
        .map(|id| {
            let x1 = rng.gen_range(-range..=range);
            let x2 = x1 + rng.gen_range(1..=sz);

            let y1 = rng.gen_range(-range..=range);
            let y2 = y1 + rng.gen_range(1..=sz);

            Rect { x1, x2, y1, y2, id }
        })
        .collect()
}
