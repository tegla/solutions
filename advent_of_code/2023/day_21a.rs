use crate::mapper::*;
pub fn run(input: &str) -> usize {
    let mut m = Map::from_iter(input.lines());
    m.dump();

    let start = m.pos_iter().find(|p| m[*p] == 'S').expect("has S");
    m[start] = '.';

    let mut next_poss = vec![start];
    let mut step = -1;
    while step < 64 {
        let ps = next_poss.clone();
        next_poss.clear();
        for p in ps {
            if m.contains_pos(p) && m[p] == '.' {
                m[p] = 'O';
                for d in DIRS {
                    next_poss.push(p + d);
                }
            }
        }
        println!("");
        m.dump();
        step += 1;
    }

    let mut result = 0;
    for p in m.pos_iter() {
        if m[p] == 'O' {
            let start_parity = (start.row() + start.column()) % 2;
            let my_parity = (p.row() + p.column() + step) % 2;
            if start_parity == my_parity {
                result += 1
            } else {
                m[p] = '.';
            }
        }
    }

    println!("");
    m.dump();

    result
}
