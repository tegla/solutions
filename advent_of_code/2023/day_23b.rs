use std::collections::{HashMap, HashSet, VecDeque};

use crate::mapper::*;

type Distances = HashMap<Pos, HashMap<Pos, usize>>;

fn calculate_distances(m: &Map, distances: &mut Distances, start: Pos, dir: Pos) {
    // println!("{start:?} {dir:?}");

    let mut cp = start + dir;
    let mut cd = dir;
    let mut dist = 1;

    loop {
        // println!("   {cp:?} {cd:?}");
        if cp == pos(m.rows() - 1, m.columns() - 2) {
            distances.entry(start).or_default().insert(cp, dist);
            distances.entry(cp).or_default().insert(start, dist);
            return;
        }

        let mut possible = vec![];
        for d in DIRS {
            if m.get(cp + d) == '#' {
                continue;
            }
            if d == cd * -1 {
                continue;
            }
            possible.push(d);
        }
        if possible.is_empty() {
            return;
        }
        if possible.len() == 1 {
            cp += possible[0];
            cd = possible[0];
            dist += 1;
        } else {
            if let Some(k) = distances.get(&start) {
                if k.contains_key(&cp) {
                    return;
                }
            }
            distances.entry(start).or_default().insert(cp, dist);
            distances.entry(cp).or_default().insert(start, dist);
            for d in possible {
                calculate_distances(m, distances, cp, d);
            }
        }
    }
}

fn is_connected(distances: &Distances, ignore_nodes: &[Pos], start: Pos, end: Pos) -> bool {
    let mut found = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(p) = q.pop_front() {
        if p == end {
            return true;
        }
        if found.contains(&p) {
            continue;
        }
        found.insert(p);
        if let Some(p2s) = distances.get(&p) {
            for p2 in p2s.keys() {
                if !ignore_nodes.contains(p2) {
                    q.push_back(*p2);
                }
            }
        }
    }
    false
}

fn longest_path(
    distances: &Distances,
    path_so_far: &[Pos],
    length_so_far: usize,
    end: Pos,
) -> usize {
    if path_so_far.len() < 7 {
        println!("{path_so_far:?}");
    }
    let last = *path_so_far.last().unwrap();
    if last == end {
        return length_so_far;
    }
    if !is_connected(distances, path_so_far, last, end) {
        return usize::MIN;
    }
    let mut new_path = Vec::from(path_so_far);
    let mut new_max = usize::MIN;
    for (p2, d) in distances.get(&last).unwrap() {
        if path_so_far.contains(p2) {
            continue;
        }
        new_path.push(*p2);
        new_max = std::cmp::max(
            new_max,
            longest_path(distances, &new_path[..], length_so_far + d, end),
        );
        new_path.pop();
    }
    return new_max;
}

pub fn run(input: &str) -> usize {
    let mut m = Map::from_iter(input.lines());
    m.set_default('#');
    for p in m.pos_iter() {
        if m[p] != '#' && m[p] != '.' {
            m[p] = '.'
        }
    }
    // m.dump();
    let start = pos(0, 1);
    let end = pos(m.rows() - 1, m.columns() - 2);

    let mut distances: Distances = HashMap::new();
    calculate_distances(&m, &mut distances, pos(0, 1), DOWN);

    for (k, v) in distances.iter() {
        m[*k] = 'X';
        println!("{k:?} {v:?}");
    }
    m.dump();

    longest_path(&distances, &[start], 0, end)
}
