use itertools::Itertools;

fn hash(s: &str) -> u8 {
    let mut h: i32 = 0;
    for c in s.chars().map(|c| c as u8) {
        h += c as i32;
        h *= 17;
        h %= 256;
    }
    println!("hash({s:?})={h:?}");
    h as u8
}

pub fn run(input: &str) -> usize {
    let commands = input.split(',').map(|s| s.trim()).collect_vec();

    let mut result: usize = 0;
    for command in commands {
        result += hash(command) as usize;
    }
    result
}
