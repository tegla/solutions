use crate::parser::*;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

fn stoer_wagner(orig: &Vec<(String, Vec<String>)>) -> usize {
    fn norm(a: usize, b: usize) -> (usize, usize) {
        if a < b {
            (a, b)
        } else if a > b {
            (b, a)
        } else {
            panic!()
        }
    }

    let mut node_names = BTreeSet::new();
    for (n, ns) in orig {
        node_names.insert(&n[..]);
        for n in ns {
            node_names.insert(&n[..]);
        }
    }
    let node_names: Vec<&str> = node_names.iter().copied().collect_vec();
    let node_count = node_names.len();
    let name_to_num: HashMap<&str, usize> =
        HashMap::from_iter(node_names.iter().enumerate().map(|(i, s)| (*s, i)));

    // Distance betwee nod i and node j, where i < j
    let mut adj_matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..node_names.len() {
        adj_matrix.push(vec![0; node_names.len()]);
    }
    let mut orig_edges = Vec::new();

    for (n1, ns) in orig.iter() {
        let n1 = name_to_num[&n1[..]];
        for n2 in ns {
            let n2 = name_to_num[&n2[..]];
            let (n1, n2) = norm(n1, n2);
            adj_matrix[n1][n2] = 1;
            orig_edges.push((n1, n2));
        }
    }
    orig_edges.sort();
    let orig_edges = orig_edges;

    // Still active node numbers in the graph, and what original nodes they represent.
    let mut nodes: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for i in 0..node_names.len() {
        nodes.insert(i, BTreeSet::from([i]));
    }

    let mut best_mincut = Vec::new();

    while nodes.len() > 2 {
        println!("Nodes: {}", nodes.len());

        // Find s,t
        // println!("Find s,t");
        let mut neighbours: Vec<HashSet<usize>> = vec![HashSet::new(); node_count];
        for i in 0..node_count {
            for j in i + 1..node_count {
                let (i, j) = norm(i, j);
                if adj_matrix[i][j] > 0 {
                    neighbours[i].insert(j);
                    neighbours[j].insert(i);
                }
            }
        }
        let neighbours = neighbours;
        // println!("have neighbours");

        let start = *nodes.keys().next().unwrap();
        let mut a: HashSet<usize> = HashSet::from_iter([start]);
        let mut candidates: HashSet<usize> = HashSet::from_iter(nodes.keys().copied());
        candidates.remove(&start);

        let mut t = usize::MAX;
        let mut s = usize::MAX;

        while !candidates.is_empty() {
            // Find the most connected-to-a vertex.
            let mut next_node = usize::MAX;
            let mut max_weight = i32::MIN;
            for n in candidates.iter().copied() {
                let mut weight = 0;
                for in_a in neighbours[n].iter().copied() {
                    if in_a == n {
                        continue;
                    }
                    if !nodes.contains_key(&in_a) {
                        continue;
                    }
                    if !a.contains(&in_a) {
                        continue;
                    }
                    let (l, r) = norm(n, in_a);
                    weight += adj_matrix[l][r];
                }
                if weight > max_weight {
                    max_weight = weight;
                    next_node = n;
                }
            }
            assert_ne!(next_node, usize::MAX);
            candidates.remove(&next_node);
            a.insert(next_node);
            s = t;
            t = next_node;
        }

        assert_ne!(s, usize::MAX);
        let s = s;
        let t = t;

        println!("Merging s={} t={}", node_names[s], node_names[t]);

        // Collect all the nodes pointing to t.
        let mut prev_t = Vec::new();
        for i in 0..node_count {
            if i == t {
                continue;
            }
            let (l, r) = norm(t, i);
            if adj_matrix[l][r] > 0 {
                prev_t.push(i);
            }
        }

        // Now expand it to the original node numbers, for cut.
        let t_orig = &nodes[&t];
        let mut prev_t_orig: HashSet<usize> = HashSet::new();
        for i in prev_t.iter() {
            prev_t_orig.extend(nodes[&i].iter());
        }
        // println!("prev_t: {:?} {:?}", prev_t, prev_t_orig);
        let mut mincut = Vec::new();
        for (l, r) in orig_edges.iter().copied() {
            if (prev_t_orig.contains(&l) && t_orig.contains(&r))
                || (prev_t_orig.contains(&r) && t_orig.contains(&l))
            {
                mincut.push((l, r));
            }
        }
        println!(
            "Mincut: {} {}",
            mincut.len(),
            mincut
                .iter()
                .map(|(l, r)| format!("{}-{}", node_names[*l], node_names[*r]))
                .join(", ")
        );
        if best_mincut.is_empty() || best_mincut.len() > mincut.len() {
            best_mincut = mincut;
        }

        // And now merge t into s.
        let ss = nodes.remove(&s).unwrap();
        let st = nodes.remove(&t).unwrap();
        let mut new_ss = ss.clone();
        new_ss.extend(st.iter());
        nodes.insert(s, new_ss);

        // Add in the adj graph t's edges to s.
        for i in 0..node_count {
            if i == t || i == s {
                continue;
            }
            let (l, r) = norm(i, t);
            let d = adj_matrix[l][r];
            let (l, r) = norm(i, s);
            adj_matrix[l][r] += d;
        }

        // And clear it from the adj graph.
        for i in 0..node_count {
            if i == t {
                continue;
            }
            let (i, t) = norm(i, t);
            adj_matrix[i][t] = 0;
        }
        println!("");
    }

    println!(
        "Best Mincut: {} {}",
        best_mincut.len(),
        best_mincut
            .iter()
            .map(|(l, r)| format!("{}-{}", node_names[*l], node_names[*r]))
            .join(", ")
    );

    let mut cut_edges: HashSet<(usize, usize)> = HashSet::new();
    for (l, r) in best_mincut.iter().copied() {
        cut_edges.insert((l, r));
        cut_edges.insert((r, l));
    }
    let cut_edges = cut_edges;

    let mut neighbours: Vec<HashSet<usize>> = vec![HashSet::new(); node_count];
    for (l, r) in orig_edges.iter().copied() {
        if cut_edges.contains(&(l, r)) {
            continue;
        }
        neighbours[l].insert(r);
        neighbours[r].insert(l);
    }

    let mut contains0 = HashSet::new();
    let mut l = vec![0];
    while let Some(n) = l.pop() {
        if contains0.contains(&n) {
            continue;
        }
        contains0.insert(n);
        l.extend(neighbours[n].iter());
    }

    println!("{} {}", contains0.len(), node_count - contains0.len());
    return contains0.len() * (node_count - contains0.len());
}

pub fn run(input: &str) -> usize {
    let v: Vec<(String, Vec<String>)> = full_parse(
        input,
        rep0(line(|s| {
            let n = word()(s)?;
            string_match(": ")(s)?;
            let ns = rep_del(word(), ws())(s)?;
            Some((n, ns))
        })),
    );

    stoer_wagner(&v)
}
