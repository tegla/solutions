use crate::parser::*;
use itertools::Itertools;
use std::iter::repeat;

fn prepend(pre: String, suff: Vec<String>) -> Vec<String> {
    suff.into_iter()
        .map(move |suff| pre.clone() + &suff[..])
        .collect_vec()
}

fn str_of(ch: char, how_many: usize) -> String {
    repeat(ch).take(how_many).join("")
}

fn possible_matches(patt: &[char], rs: &[usize]) -> Vec<String> {
    let annotate = |v: Vec<String>| {
        // println!("! {} {:?} -> {:?}", patt.iter().join(""), rs, v);
        v
    };

    if rs.len() == 0 {
        if patt.iter().all(|c| *c != '#') {
            return annotate(vec!["".to_string()]);
        } else {
            return annotate(vec![]);
        }
    }

    let rs_min_len: usize = rs.iter().sum();
    let rs_min_len = rs_min_len + rs.len() - 1;
    if patt.len() < rs_min_len {
        return annotate(vec![]);
    }

    let rs0 = rs[0] as usize;

    let mut results = Vec::new();

    let broken_after = patt.len() > rs0 && patt[rs0] == '#';
    let matches = (&patt[0..rs0]).iter().copied().all(|c| c != '.');

    if matches && !broken_after {
        let mut prefix = str_of('#', rs0);
        if patt.len() > rs0 {
            prefix.push('.');
        }
        let suffices = possible_matches(&patt[prefix.len()..], &rs[1..]);
        results.extend(prepend(prefix, suffices));
    }

    if patt.len() > 1 && patt[0] != '#' {
        let prefix = if patt[0] == '?' { '.' } else { patt[0] };
        let prefix = prefix.to_string();
        let suffices = possible_matches(&patt[1..], rs);
        results.extend(prepend(prefix, suffices));
    }

    return annotate(results);
}

fn count(patt: &String, rs: &Vec<i32>) -> usize {
    let patt = patt.chars().collect_vec();
    let rs = rs.iter().copied().map(|r| r as usize).collect_vec();
    let ms = possible_matches(&patt[..], &rs[..]);
    for m in ms.iter() {
        println!("{m}");
    }

    return ms.len() as usize;
}

pub fn run(input: &str) -> usize {
    let l = try_parse(|s| {
        let patt = rep1(char_in("?.#"))(s)?;
        let patt = patt.iter().join("");
        ws()(s)?;
        let rs = rep_del(i32(), char_in(","))(s)?;
        Some((patt, rs))
    });
    let patts = full_parse(input, rep0(line(&l)));

    let mut sum = 0;
    for (patt, rs) in patts.iter() {
        let c = count(&patt, &rs);
        println!("{patt} {} -> {c}", rs.iter().join(","));
        sum += c;
    }

    sum
}
