use std::collections::VecDeque;

use crate::mapper::*;
use itertools::Itertools;

fn dir(c: char) -> Pos {
    match c {
        'R' => RIGHT,
        'L' => LEFT,
        'U' => UP,
        'D' => DOWN,
        x => panic!("{x}"),
    }
}

pub fn run(input: &str) -> usize {
    let mut commands = vec![];
    for l in input.lines() {
        let (d, n, _) = l.split(' ').collect_tuple().unwrap();
        let d = d.chars().next().unwrap();
        let n = i32::from_str_radix(n, 10).unwrap();
        commands.push((d, n));
    }
    let commands = commands;

    let mut p = ORIGO;
    let mut ps = vec![p];
    for (d, n) in commands {
        for _ in 1..=n {
            p += dir(d);
            ps.push(p);
        }
    }
    assert_eq!(ps.first(), ps.last());

    let (topleft, bottomright) = Pos::boxing_range(ps.iter());
    let mut m = Map::empty(
        (bottomright - topleft).row() + 1,
        (bottomright - topleft).column() + 1,
    );

    for p in ps {
        m[p - topleft] = '#';
    }

    // find an inside
    let inside;
    'foo: {
        for r in m.row_range() {
            for c in m.column_range().skip(2) {
                let p = pos(r, c);
                if m[p + LEFT + LEFT] == '.' && m[p + LEFT] == '#' && m[p] == '.' {
                    inside = p;
                    break 'foo;
                }
            }
        }
        panic!("no inside?");
    }

    let mut q = VecDeque::new();
    q.push_back(inside);
    while let Some(p) = q.pop_front() {
        if m[p] == '#' {
            continue;
        }
        m[p] = '#';
        for dir in DIRS {
            if m.contains_pos(p + dir) {
                q.push_back(p + dir);
            }
        }
    }

    m.dump();

    let mut result = 0;
    for r in m.row_range() {
        for c in m.column_range() {
            if m[pos(r, c)] == '#' {
                result += 1;
            }
        }
    }

    result
}
