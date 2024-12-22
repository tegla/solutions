use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;

struct Card {
    nr: i32,
    winning: Vec<u8>,
    have: Vec<u8>,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card {:03}:", self.nr)?;
        for n in self.winning.iter() {
            write!(f, " {:02x}", n)?;
        }
        write!(f, " |")?;
        for n in self.have.iter() {
            write!(f, " {:02x}", n)?;
        }
        Ok({})
    }
}

impl Card {
    fn parse(l: &str) -> Self {
        let l = l.strip_prefix("Card ").unwrap();
        let (nr, winning_have) = l.split(":").collect_tuple().unwrap();
        let nr = i32::from_str_radix(nr.trim(), 10).unwrap();
        let (winning, have) = winning_have.split("|").collect_tuple().unwrap();
        let winning: Vec<u8> = winning
            .trim()
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect();
        let have: Vec<u8> = have
            .trim()
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect();

        Card { nr, winning, have }
    }
}

pub fn run(input: &str) -> usize {
    // Little bit of hack so that index and card is the same.
    let mut cards = vec![Card {
        nr: 0,
        winning: vec![],
        have: vec![],
    }];
    cards.extend(input.lines().map(|l| Card::parse(l)));
    let cards = cards;
    let mut counts: Vec<i32> = Vec::from_iter(cards.iter().map(|_| 1));
    counts[0] = 0;

    for c in cards.iter().skip(1) {
        let winning: HashSet<u8> = HashSet::from_iter(c.winning.iter().copied());
        let won = c
            .have
            .iter()
            .filter(|b| winning.contains(b))
            .count() as i32;
        let count = *counts.get(c.nr as usize).unwrap();
        println!("{c} - {} {}", count, won);
        for i in c.nr + 1..c.nr + won + 1 {
            counts[i as usize] += count;
        }
    }
    counts.iter().fold(0, |a, b| a + b) as usize
}
