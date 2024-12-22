use itertools::Itertools;

pub fn run(input: &str) -> usize {
    let m: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect_vec()).collect_vec();
    let mut row_expand = vec![true; m.len()];
    let mut column_expand = vec![true; m.first().unwrap().len()];

    for r in 0..m.len() {
        for c in 0..m[r].len() {
            if m[r][c] == '#' {
                row_expand[r] = false;
                column_expand[c] = false;
            }
        }
    }

    fn expanded(expand: &Vec<bool>) -> Vec<i32> {
        let mut res = vec![0; expand.len()];
        let mut r = 0;
        for i in 0..expand.len() {
            res[i] = r;
            r += 1;
            if expand[i] {
                r += 1;
            }
        }
        res
    }

    let row_expand = expanded(&row_expand);
    let column_expand = expanded(&column_expand);

    // println!("{:?}", row_expand);
    // println!("{:?}", column_expand);

    let mut stars = Vec::new();
    for r in 0..m.len() {
        for c in 0..m[r].len() {
            if m[r][c] != '#' {
                continue;
            }
            let star = (row_expand[r] as i32, column_expand[c] as i32);
            println!("{star:?}");
            stars.push(star);
        }
    }

    let mut total = 0;
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            let (ir, ic) = stars[i];
            let (jr, jc) = stars[j];
            total += i32::abs(ir - jr) + i32::abs(ic - jc);
        }
    }

    total as usize
}
