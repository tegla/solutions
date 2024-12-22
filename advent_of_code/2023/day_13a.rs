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
    for m in maps.iter() {
        let tr = id();
        let rows = mirror_row(&m.view(&tr));
        let tr = tr.flip_diagonally();
        let columns = mirror_row(&m.view(&tr));

        println!("{:?} {:?}", rows, columns);
        sum += rows.unwrap_or(0) * 100 + columns.unwrap_or(0);
    }

    sum as usize
}
