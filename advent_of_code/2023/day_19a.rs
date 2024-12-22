use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct Rule<'a> {
    cat: char,
    op: char,
    num: i32,
    action: &'a str,
}

impl Rule<'_> {
    fn apply(&self, p: &Part) -> Option<&str> {
        match self.op {
            '<' => {
                if p[&self.cat] < self.num {
                    return Some(self.action);
                }
            }
            '>' => {
                if p[&self.cat] > self.num {
                    return Some(self.action);
                }
            }
            _ => panic!(),
        }
        return None;
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    def_action: &'a str,
}

type Part = HashMap<char, i32>;

impl Workflow<'_> {
    fn next(&self, p: &Part) -> &str {
        for r in self.rules.iter() {
            if let Some(action) = r.apply(p) {
                return action;
            }
        }
        self.def_action
    }
}

fn accepted(ws: &HashMap<&str, Workflow>, p: &Part) -> bool {
    // println!("{p:?}");
    let mut wf = "in";
    while wf != "A" && wf != "R" {
        // println!("{wf}");
        wf = ws[wf].next(&p);
    }
    // println!("{wf}");
    wf == "A"
}

pub fn run(input: &str) -> usize {
    let mut workflows = HashMap::new();
    let mut input = input.lines();
    while let Some(l) = input.next() {
        if l.is_empty() {
            break;
        }
        let x = l.find('{').unwrap();
        let (name, rules_str) = (&l[0..x], &l[x + 1..l.len() - 1]);
        // println!("{name:?}, {rules_str:?}");
        let mut rules_strs = rules_str.split(',').collect_vec();
        let def_action = rules_strs.pop().unwrap();
        let mut rules = vec![];
        for rule in rules_strs {
            // println!("{rule:?}");
            let (rule, action) = rule.split_once(':').unwrap();
            let (cat, rule) = (rule.chars().next().unwrap(), &rule[1..]);
            let (op, rule) = (rule.chars().next().unwrap(), &rule[1..]);
            let num = i32::from_str_radix(rule, 10).unwrap();
            rules.push(Rule {
                cat,
                op,
                num,
                action,
            });
        }
        let w = Workflow { rules, def_action };
        // println!("{name:?} {w:?}");
        workflows.insert(name, w);
    }

    let mut parts = vec![];
    while let Some(l) = input.next() {
        // println!("{l:?}");
        let l = &l[1..l.len() - 1];
        let mut h = HashMap::new();
        for r in l.split(',') {
            let (cat, n) = r.split_once('=').unwrap();
            let cat = cat.chars().next().unwrap();
            let n = i32::from_str_radix(n, 10).unwrap();
            h.insert(cat, n);
        }
        // println!("{l:?} {h:?}");
        parts.push(h);
    }

    let mut result: i32 = 0;
    for p in parts {
        let accept = accepted(&workflows, &p);
        if accept {
            result += p.values().sum::<i32>();
        }
    }
    result as usize
}
