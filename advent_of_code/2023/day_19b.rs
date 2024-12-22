use std::collections::HashMap;
use itertools::Itertools;

use crate::multi_range::MultiRange;

const CATS: [char; 4] = ['x', 'm', 'a', 's'];

#[derive(Debug)]
struct Rule<'a> {
    cat: u8,
    op: char,
    num: i32,
    action: &'a str,
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    def_action: &'a str,
}

fn range_mask() -> MultiRange {
    MultiRange::from(1..4001)
}

type GoodCats = [MultiRange; 4];

fn count(workflows: &HashMap<&str, Workflow>, wf_name: &str, gc: &GoodCats) -> usize {
    let annotate = |result| {
        println!("{wf_name}?");
        for (i, mr) in gc.iter().enumerate() {
            println!("      {i} {mr:?}");
        }
        println!("  -> {result}");
        result
    };
    if wf_name == "A" {
        return annotate(gc.iter().map(|mr| mr.count() as usize).product());
    }
    if wf_name == "R" {
        return annotate(0);
    }
    let wf = &workflows[wf_name];
    let mut result = 0;

    let mut gc = gc.clone();
    for rule in wf.rules.iter() {
        let mask = (if rule.op == '<' {
            MultiRange::from(isize::MIN..rule.num as isize)
        } else {
            MultiRange::from(rule.num as isize + 1..isize::MAX)
        }) & range_mask();

        let mut rule_mask = gc.clone();
        rule_mask[rule.cat as usize] &= mask.clone();
        result += count(workflows, rule.action, &rule_mask);

        let mask = !mask & range_mask();
        gc[rule.cat as usize] &= mask;
    }

    result += count(workflows, wf.def_action, &gc);
    annotate(result)
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
                cat: CATS.iter().find_position(|c| **c == cat).unwrap().0 as u8,
                op,
                num,
                action,
            });
        }
        let w = Workflow { rules, def_action };
        // println!("{name:?} {w:?}");
        workflows.insert(name, w);
    }

    let gc: [_; 4] = std::array::from_fn(|_| range_mask());

    count(&workflows, "in", &gc)
}
