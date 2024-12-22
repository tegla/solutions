use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

pub fn run(input: &str) -> usize {
    let mut l = input.lines();
    let path = l.next().unwrap();
    l.next();
    let re = Regex::new("(...) = \\((...), (...)\\)").unwrap();
    let mut graph = HashMap::new();
    for l in l {
        let x = re.captures(l).unwrap();
        let (node, left, right) = (x[1].to_string(), x[2].to_string(), x[3].to_string());
        graph.insert(node, (left, right));
    }
    let graph = graph;
    let path = path.chars().collect_vec();

    let start_shift = 3;
    let end_diff = 3;
    let mut loops: Vec<i64> = Vec::new();

    for pos in graph.keys() {
        if !pos.ends_with("A") {
            continue;
        }
        let starting_pos = &pos[..];
        let mut pos = starting_pos;
        let mut path_pos = 0;

        for _ in 0..start_shift {
            let dir = path[path_pos];
            let (left, right) = graph.get(pos).unwrap();
            pos = &if dir == 'L' { left } else { right }[..];
            path_pos = (path_pos + 1) % path.len();
        }

        let mut step = 0;
        let mut reached = HashMap::new();
        reached.insert((path_pos, pos), step);
        let mut zs = Vec::new();

        loop {
            step += 1;
            let dir = path[path_pos];
            let (left, right) = graph.get(pos).unwrap();
            pos = &if dir == 'L' { left } else { right }[..];
            path_pos = (path_pos + 1) % path.len();

            if let Some(loopback) = reached.get(&(path_pos, pos)) {
                println!("{starting_pos} loop:{loopback} -> {step} - {zs:?}");
                assert_eq!(*loopback, 0);
                assert_eq!(zs.len(), 1);
                assert_eq!(step - zs[0], end_diff);

                loops.push(step);

                break;
            }

            reached.insert((path_pos, pos), step);
            if pos.ends_with("Z") {
                zs.push(step);
            }
        }
    }

    let mut rep: i64 = 1;
    for l in loops {
        rep *= l / gcd(rep, l as i64);
    }

    (rep + start_shift - end_diff) as usize
}
