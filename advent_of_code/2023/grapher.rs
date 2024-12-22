use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn calculate_distances_from<Node, Weight, FN>(
    starts: &[Node],
    f: FN,
) -> HashMap<Node, (Weight, Node)>
where
    Node: std::cmp::Eq + std::hash::Hash + std::marker::Copy,
    Weight: std::ops::Add<Weight, Output = Weight>
        + std::cmp::PartialOrd<Weight>
        + num::traits::Zero
        + std::marker::Copy,
    FN: Fn(Node) -> Vec<(Node, Weight)>,
{
    let mut distances: HashMap<Node, (Weight, Node)> = HashMap::new();
    let mut queue: VecDeque<(Node, Weight, Node)> = VecDeque::new();
    for start in starts {
        queue.push_back((*start, num::traits::Zero::zero(), *start));
    }
    while let Some((node, weight, previous)) = queue.pop_front() {
        // if distances.len() % 10000 == 0 {
        //     println!("distances size: {}", distances.len());
        // }
        let mut insert_from_edges = node == previous;
        match distances.entry(node) {
            Occupied(mut e) => {
                let current: &mut (Weight, Node) = e.get_mut();
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
