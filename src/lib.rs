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
    intersect_callback(rects, &mut |a, b| output.push((a, b)));
    output
}

fn intersect_callback(rects: &[Rect], cb: &mut impl FnMut(usize, usize)) {
    let mut v: Vec<(i32, Rect)> = rects
        .iter()
        .map(|r| [(r.x1, *r), (r.x2, *r)])
        .flatten()
        .collect();
    v.sort_by_key(|(x, _)| *x);

    detect(&v, cb)
}

fn detect(v: &[(i32, Rect)], cb: &mut impl FnMut(usize, usize)) {
    if v.len() < 2 {
        return;
    }

    let (first_half, second_half) = v.split_at(v.len() / 2);

    let mut first_h: Vec<Rect> = first_half.iter().map(|(_, r)| *r).collect();
    first_h.sort_by_key(|r| r.y1);

    let mut second_h: Vec<Rect> = second_half.iter().map(|(_, r)| *r).collect();
    second_h.sort_by_key(|r| r.y1);

    let mut s11 = vec![];
    let mut s12 = vec![];
    let mut s21 = vec![];
    let mut s22 = vec![];

    for &(_, rect) in first_half {
        if let Some((second_end, _)) = second_half.last() {
            if rect.x2 > *second_end {
                s12.push(rect);
            } else {
                s11.push(rect);
            }
        } else {
            s11.push(rect);
        }
    }

    for &(_, rect) in second_half {
        if let Some((first_begin, _)) = first_half.first() {
            if rect.x1 > *first_begin {
                s21.push(rect);
            } else {
                s22.push(rect);
            }
        } else {
            s22.push(rect);
        }
    }

    stab(&s12, &s22, cb);
    stab(&s21, &s11, cb);
    stab(&s12, &s21, cb);

    detect(&first_half, cb);
    detect(&second_half, cb);
}

fn stab(a: &[Rect], b: &[Rect], cb: &mut impl FnMut(usize, usize)) {
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i].x1 < b[j].x1 {
            let mut k = j;
            while k < b.len() && b[k].x1 < a[i].x2 {
                cb(a[i].id, b[k].id);
                k += 1;
            }
            i += 1;
        } else {
            let mut k = i;
            while k < a.len() && a[k].x1 < a[j].x2 {
                cb(b[j].id, a[k].id);
                k += 1
            }
            j += 1;
        }
    }
}


impl Rect {
    fn intersects(&self, other: &Self) -> bool {
        self.x1 <= other.x2
            && other.x1 <= self.x2
            && self.y1 <= other.y2
            && other.y1 <= self.y2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial() {
        let r1 = Rect {
            id: 0,
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8
        };

        let r2 = Rect {
            id: 1,
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8
        };

        assert!(r1.intersects(&r2));
        assert_eq!(intersect(&[r1, r2]), vec![(1, 0)]);
    }

    #[test]
    fn trivial2() {
        let r1 = Rect {
            id: 0,
            x1: 5,
            x2: 10,
            y1: 2,
            y2: 8
        };

        let r2 = Rect {
            id: 1,
            x1: 50,
            x2: 10,
            y1: 2,
            y2: 8
        };

        assert!(!r1.intersects(&r2));
        assert_eq!(intersect(&[r1, r2]), vec![]);
    }
}