use crate::mapper::*;

fn push_up(m: &mut Map) {
    for c in m.column_range() {
        let mut boulder_shift = 0;
        for r in m.row_range() {
            match m.get(pos(r, c)) {
                'O' => {
                    m.set(pos(r,c), '.');
                    m.set(pos(r - boulder_shift, c), 'O');
                }
                '.' => {
                    boulder_shift += 1;
                }
                '#' => {
                    boulder_shift = 0;
                }
                any => panic!("{any}"),
            }
        }
    }
}

fn calculate_value(m:&Map) -> isize {
    let mut result = 0;
    for r in m.row_range() {
        let value = m.rows() - r;
        for c in m.column_range() {
            if m.get(pos(r,c)) == 'O' {
                result += value;
            }
        }
    }
    return result;
}

pub fn run(input: &str) -> usize {
    let mut m = Map::from_iter(input.lines());
    m.dump();
    println!("");
    push_up(&mut m);
    m.dump();
    calculate_value(&m) as usize
}
