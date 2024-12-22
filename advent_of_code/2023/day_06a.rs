use crate::parser::*;

fn run_option_count(time: i32, distance: i32) -> i32 {
    let mut count = 0;
    for w in 1..time + 1 {
        let remaining = time - w;
        let d = remaining * w;
        if d > distance {
            count += 1;
        }
    }
    count
}

pub fn run(input: &str) -> usize {
    let (time, distance) = full_parse(input, |s| {
        string_match("Time:")(s)?;
        ws()(s)?;
        let time = line(rep_del(i32(), ws()))(s)?;
        string_match("Distance:")(s)?;
        ws()(s)?;
        let distance = line(rep_del(i32(), ws()))(s)?;
        Some((time, distance))
    });

    let mut m = 1;
    for (time, distance) in time.iter().zip(distance.iter()) {
        let count = run_option_count(*time, *distance);
        m *= count;
        println!("{count}");
    }

    m as usize
}
