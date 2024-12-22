use itertools::Itertools;
use std::fmt;
use std::collections::HashSet;


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
    let cards: Vec<Card> = input.lines().map(|l| Card::parse(l)).collect();
    let mut sum = 0;
    for c in cards.iter() {
        let winning: HashSet<u8> = HashSet::from_iter(c.winning.iter().copied());
        let won = c.have.iter().copied().filter(|b| winning.contains(b)).count() as u32;
        let point = if won == 0 { 0 } else { 2i32.pow(won-1)};
        println!("{c} - {}", point);
        sum+=point;
    }
    sum as usize
}
