mod day_01b;
mod day_02a;
mod day_02b;
mod day_03a;
mod day_03b;
mod day_04a;
mod day_04b;
mod day_05a;
mod day_05b;
mod day_06a;
mod day_06b;
mod day_07a;
mod day_07b;
mod day_08a;
mod day_08b;
mod day_09a;
mod day_09b;
mod day_10a;
mod day_10b;
mod day_11a;
mod day_11b;
mod day_12a;
mod day_12b;
mod day_13a;
mod day_13b;
mod day_14a;
mod day_14b;
mod day_15a;
mod day_15b;
mod day_16a;
mod day_16b;
mod day_17a;
mod day_17b;
mod day_18a;
mod day_18b;
mod day_19a;
mod day_19b;
mod day_20a;
mod day_20b;
mod day_21a;
mod day_21b;
mod day_22a;
mod day_22b;
mod day_23a;
mod day_23b;
mod day_24a;
mod day_25a;

mod grapher;
mod mapper;
mod multi_range;
mod parser;

use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzles: HashMap<(i32, &str), fn(&str) -> usize> = HashMap::new();
    puzzles.insert((1, "b"), day_01b::run);
    puzzles.insert((2, "a"), day_02a::run);
    puzzles.insert((2, "b"), day_02b::run);
    puzzles.insert((3, "a"), day_03a::run);
    puzzles.insert((3, "b"), day_03b::run);
    puzzles.insert((4, "a"), day_04a::run);
    puzzles.insert((4, "b"), day_04b::run);
    puzzles.insert((5, "a"), day_05a::run);
    puzzles.insert((5, "b"), day_05b::run);
    puzzles.insert((6, "a"), day_06a::run);
    puzzles.insert((6, "b"), day_06b::run);
    puzzles.insert((7, "a"), day_07a::run);
    puzzles.insert((7, "b"), day_07b::run);
    puzzles.insert((8, "a"), day_08a::run);
    puzzles.insert((8, "b"), day_08b::run);
    puzzles.insert((9, "a"), day_09a::run);
    puzzles.insert((9, "b"), day_09b::run);
    puzzles.insert((10, "a"), day_10a::run);
    puzzles.insert((10, "b"), day_10b::run);
    puzzles.insert((11, "a"), day_11a::run);
    puzzles.insert((11, "b"), day_11b::run);
    puzzles.insert((12, "a"), day_12a::run);
    puzzles.insert((12, "b"), day_12b::run);
    puzzles.insert((13, "a"), day_13a::run);
    puzzles.insert((13, "b"), day_13b::run);
    puzzles.insert((14, "a"), day_14a::run);
    puzzles.insert((14, "b"), day_14b::run);
    puzzles.insert((15, "a"), day_15a::run);
    puzzles.insert((15, "b"), day_15b::run);
    puzzles.insert((16, "a"), day_16a::run);
    puzzles.insert((16, "b"), day_16b::run);
    puzzles.insert((17, "a"), day_17a::run);
    puzzles.insert((17, "b"), day_17b::run);
    puzzles.insert((18, "a"), day_18a::run);
    puzzles.insert((18, "b"), day_18b::run);
    puzzles.insert((19, "a"), day_19a::run);
    puzzles.insert((19, "b"), day_19b::run);
    puzzles.insert((20, "a"), day_20a::run);
    puzzles.insert((20, "b"), day_20b::run);
    puzzles.insert((21, "a"), day_21a::run);
    puzzles.insert((21, "b"), day_21b::run);
    puzzles.insert((22, "a"), day_22a::run);
    puzzles.insert((22, "b"), day_22b::run);
    puzzles.insert((23, "a"), day_23a::run);
    puzzles.insert((23, "b"), day_23b::run);
    puzzles.insert((24, "a"), day_24a::run);
    puzzles.insert((25, "a"), day_25a::run);

    let puzzles = puzzles;
    let day = 24;
    let part = "a";
    let example = false;
    let input = fs::read_to_string(format!(
        "/tmp/advent_of_code/day_{:02}{}.txt",
        day,
        if example { "_example" } else { "" }
    ))?;
    let solution = *puzzles.get(&(day, &part)).unwrap();
    let output = solution(&input);
    println!("{}", output);
    return Ok(());
}
