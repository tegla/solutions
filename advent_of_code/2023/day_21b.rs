use itertools::Itertools;

use crate::mapper::*;

pub fn pad(bigmap: &Map, origmap: &Map) -> Map {
    let mut m = Map::empty(
        bigmap.rows() + origmap.rows() * 2,
        bigmap.columns() + origmap.columns() * 2,
    );
    let right_shift = pos(0, origmap.columns());
    let down_shift = pos(origmap.rows(), 0);
    for p in bigmap.pos_iter() {
        m[p + right_shift + down_shift] = bigmap[p];
    }
    let extend_count = bigmap.rows() / origmap.rows() + 2;
    for i in 0..extend_count {
        for shift in [
            right_shift * i,
            down_shift * i,
            right_shift * (extend_count - 1) + down_shift * i,
            down_shift * (extend_count - 1) + right_shift * i,
        ] {
            for pos in origmap.pos_iter() {
                m[pos + shift] = origmap[pos];
            }
        }
    }
    m
}

pub fn run(input: &str) -> usize {
    let mut m = Map::from_iter(input.lines());
    // m.dump();

    // let's make some simplifying assumptions, for calculating repeat count.
    assert_eq!(m.rows(), m.columns());
    let repeat_size = m.rows() as usize * 2;

    let start = m.pos_iter().find(|p| m[*p] == 'S').expect("has S");
    m[start] = '.';

    let orig_map = m.clone();

    let mut counts: Vec<usize> = vec![];

    let mut next_poss = vec![start];
    let mut step: isize = -1;
    let mut paritied_count: [usize; 2] = [0, 0];

    'pattern_search: loop {
        step += 1;
        let step = step as usize;
        let parity = (step as usize) % 2;

        let ps = next_poss.clone();
        next_poss.clear();
        for p in ps {
            if m.contains_pos(p) && m[p] == '.' {
                paritied_count[parity] += 1;
                m[p] = 'O';
                for d in DIRS {
                    next_poss.push(p + d);
                }
            }
        }
        if step % orig_map.rows() as usize == 0 {
            m = pad(&m, &orig_map);
            for p in next_poss.iter_mut() {
                *p += pos(orig_map.rows(), orig_map.columns());
            }
        }
        let count = paritied_count[parity];

        counts.push(count);

        let last = counts.iter().rev().copied().collect_vec();
        let diff_last = last
            .iter()
            .zip(last.iter().skip(repeat_size))
            .zip(last.iter().skip(2 * repeat_size))
            .map(|((a, b), c)| ((a - b), (b - c)))
            .collect_vec();
        let diff_diff_last = diff_last
            .iter()
            .map(|(a, b)| a - b)
            .take(2 * repeat_size)
            .collect_vec();

        if diff_diff_last.len() == 2 * repeat_size {
            let mut repeats = true;
            for i in 0..diff_diff_last.len() / 2 {
                if diff_diff_last[i] != diff_diff_last[i + diff_diff_last.len() / 2] {
                    repeats = false;
                }
            }
            if repeats {
                break 'pattern_search;
            }
        }

        // m.dump();
    }

    // Here we are, calculated up to the point where the pattern repeats itself.
    const TARGET: usize = 26501365;
    while counts.len() < TARGET + 1 {
        let n = counts.len();
        if n % 100000 == 0 {
            println!("generating {n}");
        }
        let diff1 = counts[n - repeat_size] - counts[n - 2 * repeat_size];
        let diff2 = counts[n - 2 * repeat_size] - counts[n - 3 * repeat_size];
        let dd = diff1 - diff2;
        counts.push(counts[n - repeat_size] + diff1 + dd);
    }

    counts[TARGET] as usize
}
