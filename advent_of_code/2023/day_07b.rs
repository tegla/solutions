use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn card_value(c: char) -> i32 {
    "J123456789TQKA".chars().position(|x| x == c).unwrap() as i32 + 1
}

fn lex_comp(a: &Vec<char>, b: &Vec<char>) -> Ordering {
    for (a, b) in a.iter().zip(b.iter()) {
        let va = card_value(*a);
        let vb = card_value(*b);
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

fn hand_type(hand: &Vec<char>) -> Hand {
    let mut hist = HashMap::new();
    for c in hand.iter() {
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
        _ => panic!("no type: {hand:?} {hist}"),
    }
}

fn best_hand(i: usize, hand: &mut Vec<char>, best: &mut Option<Hand>) {
    if i == hand.len() {
        let t = hand_type(&hand);
        if best.is_none() {
            *best = Some(t);
        } else {
            if t.cmp(best.as_ref().unwrap()) == Ordering::Greater {
                *best = Some(t);
            }
        }
        return;
    }

    if hand[i] != 'J' {
        best_hand(i + 1, hand, best);
        return;
    }

    for c in "123456789TQKA".chars() {
        hand[i] = c;
        best_hand(i + 1, hand, best);
        hand[i] = 'J';
    }
}

fn find_best_hand(hand: &str) -> Hand {
    let mut hand: Vec<char> = hand.chars().collect();
    let mut best: Option<Hand> = None;
    best_hand(0, &mut hand, &mut best);
    best.unwrap()
}

pub fn run(input: &str) -> usize {
    let hands: Vec<(&str, i32)> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(c, b)| (c, i32::from_str_radix(b, 10).unwrap()))
        .collect();

    let mut hands: Vec<(Hand, &str, i32)> = hands
        .iter()
        .map(|(h, b)| (find_best_hand(h), *h, *b))
        .collect();
    hands.sort_by(|(hand1, cards1, _), (hand2, cards2, _)| {
        let cmph = hand1.cmp(hand2);
        if cmph != Ordering::Equal {
            return cmph;
        } else {
            return lex_comp(&cards1.chars().collect(), &cards2.chars().collect());
        }
    });
    for x in hands.iter() {
        println!("{x:?}");
    }
    let result: i32 = hands
        .iter()
        .enumerate()
        .map(|(pos, (_, _, bid))| (pos as i32 + 1) * bid)
        .sum();
    result as usize
}
