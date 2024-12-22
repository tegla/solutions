use crate::mapper::*;
use itertools::Itertools;

fn dn(dir: Pos) -> usize {
    DIRS.iter().find_position(|d| **d == dir).unwrap().0
}

type M = Map<[bool; 4]>;

fn energize(m: &mut M, mut dir: Pos, mut pos: Pos) {
    let mut n = dn(dir);

    while m.contains_pos(pos) && !m.get_aux(pos)[n] {
        // println!("energize: {dir:?} {pos:?}");
        let c = m[pos];
        m.mut_aux(pos)[n] = true;
        match c {
            '.' => {
                pos += dir;
            }
            '|' => match dir {
                UP | DOWN => {
                    pos += dir;
                }
                LEFT | RIGHT => {
                    energize(m, UP, pos);
                    energize(m, DOWN, pos);
                    return;
                }
                _ => panic!(),
            },
            '-' => match dir {
                LEFT | RIGHT => {
                    pos += dir;
                }
                UP | DOWN => {
                    energize(m, LEFT, pos);
                    energize(m, RIGHT, pos);
                    return;
                }
                _ => panic!(),
            },
            '/' => {
                match dir {
                    LEFT => dir = DOWN,
                    RIGHT => dir = UP,
                    UP => dir = RIGHT,
                    DOWN => dir = LEFT,
                    _ => panic!(),
                }
                n = dn(dir);
                pos += dir;
            }
            '\\' => {
                match dir {
                    LEFT => dir = UP,
                    RIGHT => dir = DOWN,
                    UP => dir = LEFT,
                    DOWN => dir = RIGHT,
                    _ => panic!(),
                }
                n = dn(dir);
                pos += dir;
            }
            c => panic!("{c}"),
        }
    }
}

pub fn run(input: &str) -> usize {
    let m = Map::from_iter(input.lines());
    let mut m = m.with_aux(|_| [false; 4]);

    energize(&mut m, RIGHT, pos(0, 0));

    for r in m.row_range() {
        for c in m.column_range() {
            let p = pos(r, c);
            if m.get_aux(p).contains(&true) {
                m[p] = '#'
            }
        }
    }

    println!("final");
    m.dump();

    let mut result = 0;
    for r in m.row_range() {
        for c in m.column_range() {
            if m.get(pos(r, c)) == '#' {
                result += 1;
            }
        }
    }

    result
}
