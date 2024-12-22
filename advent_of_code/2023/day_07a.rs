use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn card_value(c: char) -> i32 {
    "123456789TJQKA".chars().position(|x| x == c).unwrap() as i32 + 1
}

fn lex_comp(a: &str, b: &str) -> Ordering {
    for (a, b) in a.chars().zip(b.chars()) {
        let va = card_value(a);
        let vb = card_value(b);
        if va < vb {
            return Ordering::Less;
        } else if va > vb {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Hand {
    // Nothing,
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

fn hand_type(hand: &str) -> Hand {
    let mut hist = HashMap::new();
    for c in hand.chars() {
        hist.entry(c).and_modify(|i| *i += 1).or_insert(1);
    }
    let hist = hist
        .values()
        .sorted()
        .map(|count| format!("{count}"))
        .join("");
    match &hist[..] {
        "1112" => Hand::One,
        "113" => Hand::Three,
        "122" => Hand::Two,
        "23" => Hand::Full,
        "14" => Hand::Four,
        "11111" => Hand::High,
        "5" => Hand::Five,
        _ => panic!("no type: {hand} {hist}"),
    }
}

fn compare_hand(a: &str, b: &str) -> Ordering {
    let ct = hand_type(a).cmp(&hand_type(b));
    if ct == Ordering::Equal {
        lex_comp(a, b)
    } else {
        ct
    }
}

pub fn run(input: &str) -> usize {
    let mut hands: Vec<(&str, i32)> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(c, b)| (c, i32::from_str_radix(b, 10).unwrap()))
        .collect();

    hands.sort_by(|(a, _), (b, _)| compare_hand(a, b));
    for (hand, bid) in hands.iter() {
        println!("{hand} {bid} {:?}", hand_type(hand));
    }
    let result: i32 = hands
        .iter()
        .enumerate()
        .map(|(pos, (_, bid))| (pos as i32 + 1) * bid)
        .sum();
    result as usize
}
