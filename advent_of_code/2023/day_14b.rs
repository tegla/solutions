use crate::mapper::*;

fn push_up(m: &mut TransformMut) {
    for c in m.column_range() {
        let mut boulder_shift = 0;
        for r in m.row_range() {
            match m.get(pos(r, c)) {
                'O' => {
                    m.set(pos(r, c), '.');
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

fn calculate_value(m: &Map) -> isize {
    let mut result = 0;
    for r in m.row_range() {
        let value = m.rows() - r;
        for c in m.column_range() {
            if m.get(pos(r, c)) == 'O' {
                result += value;
            }
        }
    }
    return result;
}

fn push_all_four_directions(m: &mut Map) {
    let mut tr = id();
    for _ in 0..4 {
        let mut mv = m.view_mut(&tr);
        push_up(&mut mv);
        tr = tr.clockwise_rot90();
    }
}

pub fn run(input: &str) -> usize {
    let mut m = Map::from_iter(input.lines());

    let mut map_states = vec![];
    map_states.push(m.clone());

    let mut loopback = 0;
    'main: for _i in 1.. {
        // println!("{_i}");
        push_all_four_directions(&mut m);

        for j in (0..map_states.len()).rev() {
            if m == map_states[j] {
                loopback = j;
                break 'main;
            }
        }
        map_states.push(m.clone());
    }

    println!("states_len={}, loopback_pos={}", map_states.len(), loopback);

    const TARGET: usize = 1000000000;

    let loop_size = map_states.len() - loopback;

    let loop_final_state = (TARGET - loopback) % loop_size + loopback;

    calculate_value(&map_states[loop_final_state]) as usize
}
