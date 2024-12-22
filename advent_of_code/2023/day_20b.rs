use std::collections::{BTreeSet, HashMap, VecDeque};

use itertools::Itertools;

#[derive(Debug)]
struct Node<'a> {
    t: char,
    o: Vec<&'a str>,
    i: Vec<&'a str>,
}

type Nodes<'a> = HashMap<&'a str, Node<'a>>;

pub fn run(input: &str) -> usize {
    let mut broadcasted = vec![];
    let mut nodes: Nodes = HashMap::new();
    for l in input.lines() {
        let (first, second) = l.split_once(" -> ").expect(l);
        let targets = second.split(", ").collect_vec();

        if first == "broadcaster" {
            broadcasted = targets;
        } else {
            nodes.insert(
                &first[1..],
                Node {
                    t: first.chars().next().expect(l),
                    o: targets,
                    i: vec![],
                },
            );
        }
    }
    let node_names = nodes.keys().copied().collect_vec();
    for k in node_names.iter().copied() {
        let os = { nodes[k].o.clone() };
        for o in os {
            nodes.get_mut(k).unwrap().i.push(o);
        }
    }
    let broadcasted = broadcasted;
    let nodes = nodes;

    let mut flipflop_high: HashMap<&str, bool> = HashMap::new();
    let mut conjunction_last_high: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    for (k, v) in nodes.iter() {
        if v.t == '&' {
            for s in node_names.iter().copied() {
                for i in nodes.get(s).expect(s).i.iter().copied() {
                    if i == *k {
                        conjunction_last_high
                            .entry(k)
                            .or_default()
                            .entry(s)
                            .or_insert(false);
                    }
                }
            }
        }
        flipflop_high.entry(k).or_insert(false);
    }

    let mut pre_rx = None;
    for (k, v) in nodes.iter() {
        if v.o == vec!["rx"] {
            assert_eq!(v.t, '&');
            pre_rx = Some(*k);
            break;
        }
    }
    let pre_rx = pre_rx.expect("can't find the single end point");

    let mut separate_sets = vec![];
    for root in broadcasted.iter().copied() {
        let mut found = BTreeSet::new();
        let mut q = VecDeque::new();
        q.push_back(root);
        while let Some(n) = q.pop_front() {
            if n == "rx" {
                continue;
            }
            if found.contains(n) {
                continue;
            }
            found.insert(n);
            let node = nodes.get(n).expect(n);
            for i in node.i.iter() {
                q.push_back(i);
            }
            for o in node.o.iter() {
                q.push_back(o);
            }
        }
        separate_sets.push(found);
    }
    for i in 0..separate_sets.len() - 1 {
        for j in i + 1..separate_sets.len() {
            let intersection = separate_sets[i]
                .intersection(&separate_sets[j])
                .copied()
                .collect_vec();
            assert_eq!(intersection, vec![pre_rx]);
        }
    }

    // Okay, we verified that it's actually four separate graphs, that get joined at pre_rx.

    let mut subtree_count = vec![];
    'subtree_loop: for subtree_start in broadcasted.iter().copied() {
        for button_count in 1.. {
            let mut pulses = VecDeque::new();

            pulses.push_back(("broadcaster", subtree_start, false));

            while let Some((source, target, high)) = pulses.pop_front() {
                println!(
                    "{source} {}-> {target}",
                    if high { "-high" } else { "-low" }
                );
                if !nodes.contains_key(target) {
                    // This should never happen now!
                    panic!("{}", target);
                }
                if target == pre_rx {
                    if high {
                        // let's just assume they were nice, and the subtree resets right at this point
                        subtree_count.push(button_count);
                        continue 'subtree_loop;
                    }
                    continue;
                }
                let node = nodes.get(target).expect(target);
                if node.t == '%' {
                    if !high {
                        let state = flipflop_high.get_mut(target).expect(target);
                        *state = !*state;
                        for o in node.o.iter().copied() {
                            pulses.push_back((target, o, *state));
                        }
                    }
                } else if node.t == '&' {
                    let last = conjunction_last_high.get_mut(target).expect(target);
                    *last.get_mut(source).expect(source) = high;
                    let all_high = last.values().all(|b| *b);
                    for o in node.o.iter().copied() {
                        pulses.push_back((target, o, !all_high));
                    }
                } else {
                    panic!();
                }
            }
        }
    }

    println!("{:?}", subtree_count);
    subtree_count.iter().product()
}
