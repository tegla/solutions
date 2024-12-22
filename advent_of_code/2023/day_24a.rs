use crate::parser::*;
use itertools::Itertools;

use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct P {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct V {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Big {
    n: i128,
    d: i128, // always pos
}

impl Big {
    fn gcd(mut a: i128, mut b: i128) -> i128 {
        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }
        a
    }

    fn new(n: i128, d: i128) -> Big {
        assert!(d > 0);
        // println!("Big {n} {d}");
        let c = Big::gcd(i128::abs(n), d);
        Big { n: n / c, d: d / c }
    }

    fn recip(self) -> Big {
        if self.n >= 0 {
            Big {
                n: self.d,
                d: self.n,
            }
        } else {
            Big {
                n: -self.d,
                d: -self.n,
            }
        }
    }

    fn abs(&self) -> Big {
        Big {
            n: i128::abs(self.n),
            d: self.d,
        }
    }
}

impl std::ops::Add<Big> for Big {
    type Output = Big;

    fn add(self, o: Big) -> Self::Output {
        Big::new(
            self.n.checked_mul(o.d).unwrap() + o.n.checked_mul(self.d).unwrap(),
            self.d.checked_mul(o.d).unwrap(),
        )
    }
}

impl std::ops::Sub<Big> for Big {
    type Output = Big;

    fn sub(self, o: Big) -> Self::Output {
        let o = Big::new(-o.n, o.d);
        self + o
    }
}

impl std::ops::Mul<Big> for Big {
    type Output = Big;

    fn mul(self, o: Big) -> Self::Output {
        let n = self.n.checked_mul(o.n).unwrap();
        let d = self.d.checked_mul(o.d).unwrap();
        Big::new(n, d)
    }
}

impl std::ops::Div<Big> for Big {
    type Output = Big;

    fn div(self, o: Big) -> Self::Output {
        self * o.recip()
    }
}

impl std::fmt::Display for Big {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.d == 1 {
            write!(f, "{}", self.n)
        } else {
            write!(f, "{}/{}", self.n, self.d)
        }
    }
}

impl std::cmp::PartialOrd for Big {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(i128::cmp(
            &(self.n.checked_mul(o.d)).unwrap(),
            &(o.n.checked_mul(self.d)).unwrap(),
        ))
    }
}

impl std::cmp::Ord for Big {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        self.partial_cmp(o).unwrap()
    }
}

fn b(n: i64) -> Big {
    Big::new(n as i128, 1)
}

fn intersect(pa: P, va: V, pb: P, vb: V) -> bool {
    assert_ne!(va.y, 0);
    assert_ne!(vb.y, 0);
    assert_ne!(va.x, 0);
    assert_ne!(vb.x, 0);
    // println!("{pa:?} {va:?}");
    // println!("{pb:?} {vb:?}");

    let ra = b(va.y) / b(va.x);
    let rb = b(vb.y) / b(vb.x);

    if ra == rb {
        // parallell
        let r = (b(pb.y) - b(pb.y)) / (b(pa.x) / b(pb.x));
        if r.abs() == ra.abs() {
            // this is the same line. No ida what the puzzle says for this.
            panic!("{pa:?}");
        }
        // println!("parallel lines");
        return false;
    }
    let x = (b(pb.y) - b(pa.y) + b(pa.x) * ra - b(pb.x) * rb) / (ra - rb);
    let y = b(pa.y) + (x - b(pa.x)) * ra;

    let a_past = if va.x > 0 { x < b(pa.x) } else { x > b(pa.x) };
    let b_past = if vb.x > 0 { x < b(pb.x) } else { x > b(pb.x) };
    if a_past || b_past {
        // println!("in the past");
        return false;
    }

    if x < b(200000000000000) || x > b(400000000000000) {
        // println!("x outside test area");
        return false;
    }
    if y < b(200000000000000) || y > b(400000000000000) {
        // println!("y outside test area");
        return false;
    }
    // println!("intersect={:.2} {:.2}", x.f(), y.f());

    return true;
}

pub fn run(input: &str) -> usize {
    let l = try_parse(|s| {
        let p = rep_del(i64(), tup2(string_match(","), ws()))(s)?;
        let (x, y, z) = p.iter().copied().collect_tuple()?;
        let p = P { x, y, z };

        ws()(s)?;
        string_match("@")(s)?;
        ws()(s)?;

        let v = rep_del(i64(), tup2(string_match(","), ws()))(s)?;
        let (x, y, z) = v.iter().copied().collect_tuple()?;
        let v = V { x, y, z };

        Some((p, v))
    });
    // let pvs = full_parse(input, rep_del(tup2(line(&l), line(&l)), line(empty())));
    let pvs = full_parse(input, rep0(line(l)));

    let mut count = 0;
    for i in 0..pvs.len() {
        let (pa, va) = pvs[i];
        for j in (i + 1)..pvs.len() {
            let (pb, vb) = pvs[j];
            if intersect(pa, va, pb, vb) {
                count += 1;
            }
        }
    }
    count as usize
}
