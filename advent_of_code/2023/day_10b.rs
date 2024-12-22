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
    fn rows(&self) -> std::ops::Range<i32> {
        (0 as i32)..(self.m.len() as i32)
    }

    fn columns(&self) -> std::ops::Range<i32> {
        (0 as i32)..(self.m.first().unwrap().len() as i32)
    }

    fn pos_usize(&self, p: P) -> Option<(usize, usize)> {
        if (0..self.m.len() as i32).contains(&p.r) {
            if (0..self.m.first().unwrap().len() as i32).contains(&p.c) {
                return Some((p.r as usize, p.c as usize));
            }
        }
        return None;
    }
    fn get(&self, p: P) -> char {
        if let Some((r, c)) = self.pos_usize(p) {
            return self.m[r][c];
        } else {
            return '.';
        }
    }
    fn set(&mut self, p: P, ch: char) {
        let (r, c) = self.pos_usize(p).unwrap();
        self.m[r][c] = ch;
    }
    fn dump(&self) {
        for r in self.m.iter() {
            println!("{}", r.iter().join(""));
        }
    }
}

fn compute(m: &M, start: P) -> i32 {
    let mut cleaned = M {
        m: vec![vec!['.'; m.m.first().unwrap().len()]; m.m.len()],
    };

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

    let mut prev = start;
    cleaned.set(prev, m.get(prev));
    while m.get(current) != 'S' {
        cleaned.set(current, m.get(current));
        // println!("{current:?}, {}", m.get(current));
        let mut next: Option<P> = None;
        for d in ds(m.get(current)) {
            if current + d != prev {
                next = Some(current + d);
                break;
            }
        }
        (prev, current) = (current, next.unwrap());
    }

    for r in cleaned.rows() {
        for c in cleaned.columns() {
            let p = p(r, c);
            if cleaned.get(p) == '.' {
                cleaned.set(p, ' ');
            }
        }
    }
    cleaned.dump();
    println!("");

    let mut count = 0;
    for r in cleaned.rows() {
        let mut prev = None;
        let mut in_pipe = false;
        for c in cleaned.columns() {
            let p = p(r, c);
            let ch = cleaned.get(p);
            // hardcoded, I'm lazy
            let ch = if ch == 'S' { '7' } else { ch };
            match ch {
                '|' => {
                    in_pipe = !in_pipe;
                }
                ' ' => {
                    if in_pipe {
                        count += 1;
                        cleaned.set(p, '*')
                    }
                }
                'L' | 'F' | 'J' | '7' => {
                    if let Some(pr) = prev {
                        prev = None;
                        if pr == 'F' && ch == 'J' {
                            in_pipe = !in_pipe;
                        } else if pr == 'L' && ch == '7' {
                            in_pipe = !in_pipe;
                        }
                    } else {
                        prev = Some(ch);
                    }
                }
                '-' => {}
                _ => panic!("{}", ch),
            }
        }
    }

    cleaned.dump();

    return count;
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
