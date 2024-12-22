use crate::mapper::*;

fn mirror_row(m: &Transform) -> Option<isize> {
    'row_loop: for r in 1..m.rows() {
        for (r1, r2) in (0..r).rev().zip(r..m.rows()) {
            for c in 0..m.columns() {
                if m.get(pos(r1, c)) != m.get(pos(r2, c)) {
                    continue 'row_loop;
                }
            }
        }
        return Some(r);
    }
    return None;
}

fn mirror_smudge_row(m: &Transform) -> Option<isize> {
    let mut same = vec![vec![false; m.rows() as usize]; m.rows() as usize];
    let mut one_diff = vec![vec![false; m.rows() as usize]; m.rows() as usize];

    for r1 in 0..m.rows() - 1 {
        for r2 in r1..m.rows() {
            let mut diff_count = 0;
            for c in 0..m.columns() {
                if m.get(pos(r1, c)) != m.get(pos(r2, c)) {
                    diff_count += 1;
                }
            }
            let r1 = r1 as usize;
            let r2 = r2 as usize;
            match diff_count {
                0 => {
                    same[r1][r2] = true;
                    same[r2][r1] = true;
                    // println!("same row: {r1} {r2}");
                }
                1 => {
                    one_diff[r1][r2] = true;
                    one_diff[r2][r1] = true;
                    // println!("one_diff row: {r1} {r2}");
                }
                _ => {}
            }
        }
    }
    let same = same;

    let mut result = Vec::new();

    for r1 in 0..one_diff.len() - 1 {
        'r2_loop: for r2 in r1..one_diff[r1].len() {
            // if they are mirrored, they must be odd distance from each other
            if (r2 - r1) % 2 == 0 {
                continue 'r2_loop;
            }
            if !one_diff[r1][r2] {
                continue 'r2_loop;
            }

            let middle = (r1 + r2 + 1) / 2;

            // println!("comparing: {middle}");

            'compare_loop: for (m1, m2) in (0..middle).rev().zip(middle..same.len()) {
                // println!("comparing: {middle} {m1} {m2}");
                if m1 == r1 {
                    // these are the different ones
                    assert_eq!(m2, r2);
                    continue 'compare_loop;
                }
                if !same[m1][m2] {
                    // println!("not same: {middle} {m1} {m2}");
                    continue 'r2_loop;
                }
            }

            result.push(middle);
        }
    }

    assert!(result.len() < 2);
    if result.is_empty() {
        return None;
    } else {
        return Some(result[0] as isize);
    }
}

pub fn run(input: &str) -> usize {
    let mut maps: Vec<Map> = Vec::new();
    let mut ls: Vec<&str> = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            maps.push(Map::from(ls.clone()));
            ls.clear();
        } else {
            ls.push(l);
        }
    }
    maps.push(Map::from(ls.clone()));

    let mut sum = 0;
    for (i, m) in maps.iter().enumerate() {
        let tr = id();
        let row_view = m.view(&tr);
        let orig_rows = mirror_row(&row_view);
        let rows = mirror_smudge_row(&m.view(&tr));

        let tr = id().flip_diagonally();
        let column_view = m.view(&tr);
        let orig_columns = mirror_row(&column_view);
        let columns = mirror_smudge_row(&column_view);

        if rows.is_some() == columns.is_some() {
            panic!(
                "map {i}:   row={:?} column={:?};   old_row={:?} old_column={:?}",
                rows, columns, orig_rows, orig_columns
            );
        }

        println!("{:?} {:?}", rows, columns);
        sum += rows.unwrap_or(0) * 100 + columns.unwrap_or(0);
    }

    sum as usize
}
