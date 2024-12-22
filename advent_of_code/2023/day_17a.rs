use crate::mapper::*;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Node {
    at: Pos,
    dir: Pos,
    blocks: i32,
}

type Weight = i32;

fn calculate_distances_from<FN: Fn(Node) -> Vec<(Node, Weight)>>(
    start: Node,
    f: FN,
) -> HashMap<Node, (Weight, Node)> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, start));
    while let Some((node, weight, previous)) = queue.pop_front() {
        if distances.len() % 10000 == 0 {
            println!("distances size: {}", distances.len());
        }
        let mut insert_from_edges = node == previous;
        match distances.entry(node) {
            Occupied(mut e) => {
                let current: &mut (i32, Node) = e.get_mut();
                if current.0 > weight {
                    *current = (weight, previous);
                    insert_from_edges = true;
                }
            }
            Vacant(e) => {
                e.insert((weight, previous));
                insert_from_edges = true;
            }
        }
        if insert_from_edges {
            for (new_node, plus_weight) in f(node) {
                let new_weight = weight + plus_weight;
                if let Some((w, _)) = distances.get(&new_node) {
                    if new_weight < *w {
                        queue.push_back((new_node, new_weight, node));
                    }
                } else {
                    queue.push_back((new_node, new_weight, node));
                }
            }
        }
    }
    distances
}

pub fn run(input: &str) -> usize {
    let m = Map::from_iter(input.lines());

    let dist = |n1: Node| -> Vec<(Node, Weight)> {
        let mut res = vec![];
        'd2_loop: for d2 in DIRS {
            let p2 = n1.at + d2;
            // println!("A {:?} {:?}", d2, p2);
            if d2 + n1.dir == ORIGO {
                // no turn back allowed
                continue 'd2_loop;
            }
            // println!("B {:?} {:?}", d2, p2);
            if !m.contains_pos(p2) {
                continue 'd2_loop;
            }
            let weight = m[p2].to_digit(10).unwrap() as i32;
            // println!("C {:?} {:?}", d2, p2);
            if d2 == n1.dir {
                if n1.blocks < 3 {
                    res.push((
                        Node {
                            at: p2,
                            dir: d2,
                            blocks: n1.blocks + 1,
                        },
                        weight,
                    ));
                }
            } else {
                res.push((
                    Node {
                        at: p2,
                        dir: d2,
                        blocks: 1,
                    },
                    weight,
                ));
            }
        }
        res
    };

    let distances = calculate_distances_from(
        Node {
            at: pos(0, 0),
            dir: RIGHT,
            blocks: 0,
        },
        dist,
    );

    let mut min_weight = i32::MAX;
    for (node, (weight, _)) in distances {
        let p = node.at;
        if p.row() + 1 == m.rows() && p.column() + 1 == m.columns() {
            min_weight = std::cmp::min(min_weight, weight);
        }
    }

    min_weight as usize
}
