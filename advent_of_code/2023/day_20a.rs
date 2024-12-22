use std::collections::{HashMap, VecDeque};

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

    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 1..=1000 {
        let mut pulses = VecDeque::new();

        pulses.push_back(("button", "broadcaster", false));

        while let Some((source, target, high)) = pulses.pop_front() {
            if high {
                high_count += 1;
            } else {
                low_count += 1;
            }
            println!(
                "{source} {}-> {target}",
                if high { "-high" } else { "-low" }
            );
            if target == "broadcaster" {
                for p in broadcasted.iter().copied() {
                    pulses.push_back(("broadcaster", p, false));
                }
                continue;
            }
            if !nodes.contains_key(target) {
                println!("  {target} was an output node.");
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
    println!("low={low_count} high={high_count}");
    high_count * low_count
}
