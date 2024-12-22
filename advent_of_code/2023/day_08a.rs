use std::{collections::HashMap, str::Chars};

use itertools::Itertools;
use regex::Regex;

struct PathFinder<'a> {
    graph: &'a HashMap<String, (String, String)>,
    pos: &'a str,
    path: &'a str,
    i: Chars<'a>,
}

impl<'a> Iterator for PathFinder<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.i.next() {
            Some(c) => c,
            None => {
                self.i = self.path.chars();
                self.i.next().unwrap()
            }
        };

        // println!("{}", self.pos);
        let (left, right) = self.graph.get(self.pos).unwrap();

        match c {
            'L' => self.pos = &left[..],
            'R' => self.pos = &right[..],
            _ => panic!("{c}"),
        }

        Some(self.pos)
    }
}

impl<'a> PathFinder<'a> {
    fn new(
        graph: &'a HashMap<String, (String, String)>,
        path: &'a str,
        pos: &'a str,
    ) -> PathFinder<'a> {
        PathFinder {
            graph,
            pos,
            path,
            i: path.chars(),
        }
    }
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

    let (steps, _) = PathFinder::new(&graph, path, "AAA")
        .find_position(|pos| *pos == "ZZZ")
        .unwrap();

    steps + 1
}
