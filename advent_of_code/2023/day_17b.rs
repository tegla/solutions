use crate::grapher::calculate_distances_from;
use crate::mapper::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Node {
    at: Pos,
    dir: Pos,
    blocks: i32,
}

pub fn run(input: &str) -> usize {
    let m = Map::from_iter(input.lines());

    let dist = |n1: Node| -> Vec<(Node, i32)> {
        let mut res = vec![];
        'd2_loop: for d2 in DIRS {
            let p2 = n1.at + d2;
            if d2 + n1.dir == ORIGO {
                // no turn back allowed
                continue 'd2_loop;
            }
            if !m.contains_pos(p2) {
                // Off the edge of map.
                continue 'd2_loop;
            }
            let weight = m[p2].to_digit(10).unwrap() as i32;
            if d2 == n1.dir {
                if n1.blocks < 10 {
                    // Must turn.
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
                if n1.blocks >= 4 {
                    // Must not turn.
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
        }
        res
    };

    let distances = calculate_distances_from(
        &[
            Node {
                at: pos(0, 0),
                dir: RIGHT,
                blocks: 0,
            },
            Node {
                at: pos(0, 0),
                dir: DOWN,
                blocks: 0,
            },
        ],
        dist,
    );

    let mut min_weight = i32::MAX;
    let bottom_right = pos(m.rows() - 1, m.columns() - 1);
    for (node, (weight, _)) in distances {
        if node.at == bottom_right {
            min_weight = std::cmp::min(min_weight, weight);
        }
    }

    min_weight as usize
}
