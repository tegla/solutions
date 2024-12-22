use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
struct Game<'a> {
    #[allow(dead_code)]
    nr: i32,
    grabs: Vec<HashMap<&'a str, i32>>,
}

impl Game<'_> {
    fn parse(line: &str) -> Game {
        let line = line.strip_prefix("Game ").unwrap();
        let (nr, grab_strs) = line.split_once(':').unwrap();
        let nr = i32::from_str_radix(nr, 10).unwrap();
        let mut grabs: Vec<HashMap<&str, i32>> = Vec::new();
        for grab_str in grab_strs.split(";").into_iter() {
            let mut rgb: HashMap<&str, i32> = HashMap::new();
            for color_str in grab_str.split(",") {
                let color_str = color_str.trim();
                let mut parts = color_str.split_ascii_whitespace();
                let num = i32::from_str_radix(parts.next().unwrap(), 10).unwrap();
                let color_str = parts.next().unwrap();
                assert_eq!(parts.next(), None);
                rgb.insert(color_str, num);
            }
            grabs.push(rgb);
        }
        Game { nr, grabs }
    }

    fn minimal_set(&self) -> HashMap<&str, i32> {
        let mut result: HashMap<&str, i32> = HashMap::new();
        for grab in self.grabs.iter() {
            for (color, num) in grab.iter() {
                let grabbed_num: i32 = num.clone();
                let max_num = result.get(color).cloned().unwrap_or(0);
                if grabbed_num > max_num {
                    result.insert(color, grabbed_num);
                }
            }
        }
        result
    }
}

pub fn run(input: &str) -> usize {
    let mut sum = 0;
    for l in input.lines() {
        let game = Game::parse(l);
        let minimal_set = game.minimal_set();
        let power = minimal_set.values().fold(1, |x, y| x * y);
        println!("{power} {game:?}");
        sum += power
    }
    sum as usize
}
