use crate::parser::*;
use itertools::Itertools;

fn winning(time: i64, distance: i64, wait: i64) -> bool {
    let remaining = time - wait;
    let d = remaining * wait;
    return d > distance;
}

fn run_option_count(time: i64, distance: i64) -> i64 {
    let (mut min, mut max) = (1, time);
    while min + 1 < max {
        let middle = (min + max) / 2;
        if winning(time, distance, middle) {
            min = middle;
        } else {
            max = middle;
        }
    }

    let top = min;
    println!("{}", top);

    let (mut min, mut max) = (1, top - 1);
    while min + 1 < max {
        let middle = (min + max) / 2;
        if !winning(time, distance, middle) {
            min = middle;
        } else {
            max = middle;
        }
    }

    let bottom: i64 = min;
    println!("{}", bottom);
    return top - bottom;
}

pub fn run(input: &str) -> usize {
    let (time, distance) = full_parse(input, |s| {
        string_match("Time:")(s)?;
        ws()(s)?;
        let time = line(rep_del(i32(), ws()))(s)?;
        let time = time.iter().map(|s| format!("{}", s)).join("");
        string_match("Distance:")(s)?;
        ws()(s)?;
        let distance = line(rep_del(i32(), ws()))(s)?;
        let distance = distance.iter().map(|s| format!("{}", s)).join("");
        Some((time, distance))
    });
    let time = i64::from_str_radix(&time, 10).unwrap();
    let distance = i64::from_str_radix(&distance, 10).unwrap();

    println!("{} {}", time, distance);
    run_option_count(time, distance) as usize
}
