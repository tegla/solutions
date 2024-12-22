use itertools::Itertools;

struct M {
    m: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct P {
    r: i32,
    c: i32,
}

fn p(r: i32, c: i32) -> P {
    P { r, c }
}

impl std::ops::Add for P {
    type Output = P;

    fn add(self, rhs: Self) -> Self::Output {
        P {
            r: self.r + rhs.r,
            c: self.c + rhs.c,
        }
    }
}

impl std::ops::Sub for P {
    type Output = P;

    fn sub(self, rhs: Self) -> Self::Output {
        P {
            r: self.r - rhs.r,
            c: self.c - rhs.c,
        }
    }
}

const UP: P = P { r: -1, c: 0 };
const DOWN: P = P { r: 1, c: 0 };
const LEFT: P = P { r: 0, c: -1 };
const RIGHT: P = P { r: 0, c: 1 };
const DIRS: [P; 4] = [UP, DOWN, LEFT, RIGHT];

fn ds(c: char) -> Vec<P> {
    match c {
        '|' => vec![UP, DOWN],    // vertical pipe connecting north and south.
        '-' => vec![LEFT, RIGHT], // horizontal pipe connecting east and west.
        'L' => vec![UP, RIGHT],   // 90-degree bend connecting north and east.
        'J' => vec![UP, LEFT],    // 90-degree bend connecting north and west.
        '7' => vec![LEFT, DOWN],  // 90-degree bend connecting south and west.
        'F' => vec![DOWN, RIGHT], // 90-degree bend connecting south and east.
        '.' => vec![],            // ground; there is no pipe in this tile.
        'S' => vec![],            // starting pos, ignore
        _ => panic!("{}", c),
    }
}

impl M {
    fn get(&self, p: P) -> char {
        if (0..self.m.len() as i32).contains(&p.r) {
            let v = self.m.get(p.r as usize).unwrap();
            if (0..v.len() as i32).contains(&p.c) {
                return *v.get(p.c as usize).unwrap();
            }
        }
        return '.';
    }
}

fn compute(m: &M, start: P) -> i32 {
    let mut current: Option<P> = None;
    for d in DIRS {
        for d2 in ds(m.get(start + d)) {
            if d + d2 == p(0, 0) {
                current = Some(start + d);
                break;
            }
        }
    }
    let mut current = current.unwrap();

    let mut count = 1;
    let mut prev = start;
    while m.get(current) != 'S' {
        println!("{current:?}, {}", m.get(current));
        let mut next: Option<P> = None;
        for d in ds(m.get(current)) {
            if current + d != prev {
                next = Some(current + d);
                break;
            }
        }
        count += 1;
        (prev, current) = (current, next.unwrap());
    }
    return count / 2;
}

pub fn run(input: &str) -> usize {
    let m = M {
        m: input.lines().map(|l| l.chars().collect_vec()).collect_vec(),
    };
    let mut i = 0;
    let mut j = 0;
    for (r, v) in m.m.iter().enumerate() {
        for (c, ch) in v.iter().enumerate() {
            if *ch == 'S' {
                i = r;
                j = c;
            }
        }
    }

    compute(&m, p(i as i32, j as i32)) as usize
}
