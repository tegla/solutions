use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use crate::mapper::*;

fn dir(c: char) -> Pos {
    match c {
        '0' => RIGHT,
        '2' => LEFT,
        '3' => UP,
        '1' => DOWN,
        x => panic!("{x}"),
    }
}

pub fn run(input: &str) -> usize {
    let mut commands = vec![];
    for l in input.lines() {
        let (_, _, n) = l.split(' ').collect_tuple().unwrap();
        let n = &n[2..n.len() - 1];
        let (n, d) = (&n[0..5], *&n[5..].chars().next().unwrap());
        // println!("{n:?}, {d:?}");
        let n = i32::from_str_radix(n, 16).unwrap();
        commands.push((d, n));
    }
    let commands = commands;

    let mut p = ORIGO;
    let mut ps = vec![p];
    for (d, n) in commands.iter().copied() {
        // println!("{d} {n}");
        p += dir(d) * n;
        ps.push(p);
    }
    assert_eq!(ps.first(), ps.last());
    let ps = ps;

    let mut rs = BTreeSet::new();
    let mut cs = BTreeSet::new();

    rs.extend(ps.iter().map(&Pos::row).collect_vec());
    cs.extend(ps.iter().map(&Pos::column).collect_vec());

    println!("rows: {rs:?}");
    let mut row_weights = Vec::new();
    let mut row_pos = BTreeMap::new();
    for (r1, r2) in rs.iter().zip(rs.iter().skip(1)) {
        row_pos.insert(r1, row_weights.len() as isize);
        row_weights.push(1);
        row_weights.push(r2 - r1 - 1);
    }
    row_pos.insert(rs.last().unwrap(), row_weights.len() as isize);
    row_weights.push(1);
    let row_weights = row_weights;
    let row_pos = row_pos;
    println!("{row_weights:?}");
    println!("{row_pos:?}");

    println!("columns: {cs:?}");
    let mut column_weights = Vec::new();
    let mut column_pos = BTreeMap::new();
    for (c1, c2) in cs.iter().zip(cs.iter().skip(1)) {
        column_pos.insert(c1, column_weights.len() as isize);
        column_weights.push(1);
        column_weights.push(c2 - c1 - 1);
    }
    column_pos.insert(cs.last().unwrap(), column_weights.len() as isize);
    column_weights.push(1);
    let column_weights = column_weights;
    let row_weights = row_weights;
    println!("{column_weights:?}");
    println!("{column_pos:?}");

    let to_smallmap_pos = |p: Pos| pos(row_pos[&p.row()], column_pos[&p.column()]);

    let mut m = Map::empty(row_weights.len() as isize, column_weights.len() as isize);

    let mut p_bigmap = ORIGO;
    let mut p_smallmap: Pos = to_smallmap_pos(p_bigmap);
    m[p_smallmap] = '#';
    for (d, n) in commands {
        println!("at: {p_bigmap:?} {p_smallmap:?}");
        p_bigmap += dir(d) * n;
        println!("go to: {p_bigmap:?}");
        while p_smallmap != to_smallmap_pos(p_bigmap) {
            println!("at: {p_bigmap:?} {p_smallmap:?}");
            p_smallmap += dir(d);
            m[p_smallmap] = '#';
        }
    }

    // m.dump();

    // find an inside
    let inside;
    'inside_loop: {
        for r in m.row_range() {
            for c in m.column_range() {
                let p = pos(r, c);
                if m.get(p + LEFT + LEFT) == '.' && m.get(p + LEFT) == '#' && m.get(p) == '.' {
                    inside = p;
                    break 'inside_loop;
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

    // m.dump();

    let mut result = 0;
    for r in m.row_range() {
        for c in m.column_range() {
            if m[pos(r, c)] == '#' {
                result += row_weights[r as usize] * column_weights[c as usize];
            }
        }
    }

    result as usize
}
