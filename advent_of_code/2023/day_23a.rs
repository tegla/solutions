use crate::mapper::*;

fn dir_for(c: char) -> Pos {
    match c {
        '>' => RIGHT,
        '<' => LEFT,
        '^' => UP,
        'v' => DOWN,
        c => panic!("{}", c),
    }
}

fn max_steps(m: &Map, start: Pos, current: usize) -> usize {
    println!("{start:?} {current} {}", m[start]);
    // m.dump();
    // println!("");
    let mut m = m.clone();

    let mut p = start;
    let mut current = current;

    loop {
        let underneath = m[p];
        // println!("  {p:?} {current} {underneath}");
        m[p] = 'O';
        if p == pos(m.rows() - 1, m.columns() - 2) {
            return current;
        }

        let proposals: Vec<Pos>;
        if underneath == '.' {
            proposals = Vec::from(DIRS);
        } else {
            proposals = vec![dir_for(underneath)];
        }
        let mut possible = vec![];
        for d in proposals {
            match m.get(p + d) {
                '#' => {}
                'O' => {}
                '.' => possible.push(d),
                c => {
                    if dir_for(c) == d {
                        possible.push(d);
                    }
                }
            }
        }
        if possible.is_empty() {
            return usize::MIN;
        }
        if possible.len() == 1 {
            p += possible[0];
            current += 1;
        } else {
            let mut result = usize::MIN;
            for d in possible {
                result = std::cmp::max(result, max_steps(&m, p + d, current + 1));
            }
            return result;
        }
    }
}

pub fn run(input: &str) -> usize {
    let mut m = Map::from_iter(input.lines());
    m.set_default('#');
    m.dump();

    max_steps(&m, pos(0, 1), 0)
}
