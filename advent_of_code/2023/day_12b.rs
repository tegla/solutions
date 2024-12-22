use crate::parser::*;
use itertools::Itertools;
use memoize::memoize;

fn can_fit(patt: &[char], rs: &[usize]) -> bool {
    if rs.len() == 0 {
        return true;
    }
    let rs_min_len: usize = rs.iter().sum();
    let rs_min_len = rs_min_len + rs.len() - 1;

    if patt.len() < rs_min_len {
        return false;
    } else {
        return true;
    }
}

#[memoize]
fn count_full_question_simple(patt_len: usize, rs_len: usize) -> usize {
    // How many matches do you get with ???? of len patt_len and [1,1,1,1] of len rs_len ?
    // println!("{patt_len} {rs_len}");
    assert_ne!(rs_len, 0);
    if rs_len == 1 {
        return patt_len;
    }
    let min_patt_len = rs_len * 2 - 1;
    if min_patt_len > patt_len {
        return 0;
    }
    if min_patt_len == patt_len {
        return 1;
    }

    let mut count = 0;
    for i in 0..patt_len - min_patt_len + 1 {
        // println!("{patt_len} {rs_len} {i}?");
        count += count_full_question_simple(patt_len - 2 - i, rs_len - 1);
    }
    // println!("{patt_len} {rs_len} -> {count}");
    return count;
}

fn count_full_question(patt_len: usize, rs: &[usize]) -> usize {
    if rs.len() == 0 {
        return 1;
    }
    let rs_sum: usize = rs.iter().sum();
    return count_full_question_simple(patt_len + rs.len() - rs_sum, rs.len());
}

fn count_split_by_hash(patt: &[char], rs: &[usize]) -> usize {
    // println!("hash {} {:?} ?", patt.iter().join(""), rs);
    // assumes no dots in patt
    let annotate = |r: usize| {
        // println!("hash {} {:?} -> {}", patt.iter().join(""), rs, r);
        r
    };

    if rs.len() == 0 {
        if patt.iter().all(|c| *c == '?') {
            return annotate(1);
        } else {
            return annotate(0);
        }
    }

    if !can_fit(patt, rs) {
        return annotate(0);
    }

    if let Some((hash_begin, _)) = patt.iter().copied().find_position(|c| *c == '#') {
        let hash_len = (&patt[hash_begin..])
            .iter()
            .copied()
            .take_while(|c| *c == '#')
            .count();
        let mut sum = 0;
        // println!("first_hash: {hash_begin} {hash_len}");
        'try_rs: for r in 0..rs.len() {
            let rs_r = rs[r];
            if rs_r < hash_len {
                continue 'try_rs;
            }
            let shifts: usize = rs_r - hash_len + 1;
            // println!("shifts: {r} {shifts}");
            'try_shifts: for shift in (0..shifts).rev() {
                if hash_begin < shift {
                    continue 'try_shifts;
                }
                let shift_begin = hash_begin - shift;
                let shift_end = shift_begin + rs_r;
                if patt.len() < shift_end {
                    continue 'try_shifts;
                }

                // println!("proposed #: {shift_begin}..{shift_end}");

                // left and right side of the match, with surrounding ?'s
                let patt_begin;
                if shift_begin > 0 {
                    patt_begin = shift_begin - 1;
                    if patt[patt_begin] == '#' {
                        continue 'try_shifts;
                    }
                } else {
                    patt_begin = shift_begin;
                }
                let patt_end;
                if shift_end < patt.len() {
                    patt_end = shift_end + 1;
                    if patt[patt_end - 1] == '#' {
                        continue 'try_shifts;
                    }
                } else {
                    patt_end = shift_end;
                }

                let left_patt = &patt[0..patt_begin];
                let right_patt = &patt[patt_end..];
                // println!(
                //     "Split result: {}-{}-{}",
                //     left_patt.iter().join(""),
                //     (&patt[shift_begin..shift_end]).iter().join(""),
                //     right_patt.iter().join(""),
                // );
                let left_rs = &rs[0..r];
                let right_rs = &rs[r + 1..];
                if !can_fit(left_patt, left_rs) || !can_fit(right_patt, right_rs) {
                    continue 'try_shifts;
                }
                let left_matches = count_split_by_hash(left_patt, left_rs);
                let right_matches = count_split_by_hash(right_patt, right_rs);
                let mul_count = left_matches * right_matches;
                // println!(
                //     "!! {}/{} {left_rs:?}/{right_rs:?} -> {left_matches}*{right_matches}={}",
                //     left_patt.iter().join(""),
                //     right_patt.iter().join(""),
                //     mul_count,
                // );
                sum += mul_count;
            }
        }
        return annotate(sum);
    }

    // this is all "????"
    return annotate(count_full_question(patt.len(), rs));
}

fn count_split_by_dot(patt: &[char], rs: &[usize]) -> usize {
    let annotate = |r: usize| {
        // println!("dot {} {:?} -> {}", patt.iter().join(""), rs, r);
        r
    };

    let mut patt = patt;
    while patt.len() > 0 && patt[0] == '.' {
        patt = &patt[1..];
    }

    while patt.len() > 0 && patt[patt.len() - 1] == '.' {
        patt = &patt[0..patt.len() - 1];
    }
    let patt = patt;

    if rs.len() == 0 {
        if patt.iter().all(|c| *c != '#') {
            return annotate(1);
        } else {
            return annotate(0);
        }
    }

    if !can_fit(patt, rs) {
        return annotate(0);
    }

    if let Some((dot, _)) = patt.iter().copied().find_position(|c| *c == '.') {
        assert!(patt.len() > 2); // we trimmed already.
        let mut sum = 0;
        for i in 0..rs.len() + 1 {
            let left_rs = &rs[0..i];
            let right_rs = &rs[i..rs.len()];
            let left_patt = &patt[0..dot];
            let right_patt = &patt[dot + 1..];
            if can_fit(left_patt, left_rs) && can_fit(right_patt, right_rs) {
                let left_matches = count_split_by_dot(left_patt, left_rs);
                if left_matches > 0 {
                    let right_matches = count_split_by_dot(right_patt, right_rs);
                    // println!(
                    //     "!? {}/{} {left_rs:?}/{right_rs:?}",
                    //     left_patt.iter().join(""),
                    //     right_patt.iter().join(""),
                    // );
                    sum += left_matches * right_matches;
                    // println!(
                    //     "!! {}/{} {left_rs:?}/{right_rs:?} -> {left_matches}*{right_matches}={}",
                    //     left_patt.iter().join(""),
                    //     right_patt.iter().join(""),
                    //     left_matches * right_matches,
                    // );
                }
            }
        }
        return annotate(sum);
    }

    assert!(patt.iter().copied().all(|c| c == '#' || c == '?'));

    return count_split_by_hash(patt, rs);
}

fn count(patt: &String, rs: &Vec<i32>) -> usize {
    let mut p2: Vec<char> = Vec::new();
    let mut rs2: Vec<usize> = Vec::new();
    for _ in 0..5 {
        p2.extend(patt.chars());
        p2.push('?');
        rs2.extend(rs.iter().map(|r| *r as usize));
    }
    p2.pop();
    let ms = count_split_by_dot(&p2[..], &rs2[..]);
    // for m in ms.iter() {
    //     println!("{m}");
    // }

    return ms;
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
        // println!("{patt} {}?", rs.iter().join(","));
        let c = count(&patt, &rs);
        println!("{patt} {} -> {c}", rs.iter().join(","));
        sum += c;
    }

    sum
}
