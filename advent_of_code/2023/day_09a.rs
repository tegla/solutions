use crate::parser::*;

pub fn run(input: &str) -> usize {
    let data: Vec<Vec<i64>> = full_parse(input, rep0(line(rep_del(i64(), ws()))));

    let sum: i64 = data.iter().map(triangle).sum();

    sum as usize
}

fn triangle(d: &Vec<i64>) -> i64 {
    // println!("{d:?}");
    let mut tr: Vec<Vec<i64>> = Vec::new();
    tr.push(d.clone());
    while tr.last().unwrap().len() > 1 {
        let l = tr.last().unwrap();
        let mut n = vec![0; l.len() - 1];
        for i in 0..n.len() {
            n[i] = l[i + 1] - l[i];
        }
        // println!("{n:?}");
        tr.push(n);
    }

    let mut s = 0;
    for i in (0..tr.len()).rev() {
        s = s + tr[i].last().unwrap();
    }
    // println!("{s}");
    s
}
