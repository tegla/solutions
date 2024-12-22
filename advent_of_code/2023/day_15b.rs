use itertools::Itertools;

fn hash(s: &str) -> u8 {
    let mut h: i32 = 0;
    for c in s.chars().map(|c| c as u8) {
        h += c as i32;
        h *= 17;
        h %= 256;
    }
    // println!("hash({s:?})={h:?}");
    h as u8
}

pub fn run(input: &str) -> usize {
    let commands = input.split(',').map(|s| s.trim()).collect_vec();

    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];

    for command in commands {
        if command.contains("=") {
            let (label, focal) = command.split("=").collect_tuple().unwrap();
            let focal = u8::from_str_radix(focal, 10).unwrap();
            let i = hash(label);
            let mut b: Vec<(&str, u8)> = boxes[i as usize].clone();
            if let Some((pos, _)) = b.iter().find_position(|(l, _)| *l == label) {
                b[pos] = (label, focal);
            } else {
                b.push((label, focal));
            }
            println!("{command}: {i} -> {b:?}");
            boxes[i as usize] = b;
        } else {
            assert!(command.contains("-"));
            let (label, _) = command.split("-").collect_tuple().unwrap();
            let i = hash(label);
            let b: Vec<(&str, u8)> = boxes[i as usize].clone();
            boxes[i as usize] = b.iter().filter(|(l, _)| *l != label).copied().collect_vec();
            println!("{command}: {i} -> {b:?}");
        }
    }

    let mut sum = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            let (label, focal) = boxes[i][j];

            let value = (i + 1) * (j + 1) * (focal as usize);

            println!("box {i}, label {label}, focal {focal}  => {value}");
            sum += value;
        }
    }
    sum
}
