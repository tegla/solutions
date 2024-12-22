use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let numbers = HashMap::from([
        //("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut sum = 0;
    for l in input.lines() {
        let mut first = None;
        let mut last = None;
        let mut sub = l;
        while !sub.is_empty() {
            'number: for (k, v) in numbers.iter() {
                if sub.starts_with(k) {
                    first.get_or_insert(v);
                    last.replace(v);
                    break 'number;
                }
            }
            sub = &sub[1..];
        }
        let linenum = first.unwrap() * 10 + last.unwrap();
        sum += linenum;
        println!("{l} -> {linenum}");
    }
    sum as usize
}
