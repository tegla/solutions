use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
struct Game {
    nr: i32,
    grabs: Vec<HashMap<String, i32>>,
}

impl Game {
    fn parse(line: &str) -> Game {
        let line = line.strip_prefix("Game ").unwrap();
        let (nr, grab_strs) = line.split_once(':').unwrap();
        let nr = i32::from_str_radix(nr, 10).unwrap();
        let mut grabs = Vec::new();
        for grab_str in grab_strs.split(";").into_iter() {
            let mut rgb = HashMap::new();
            for color_str in grab_str.split(",") {
                let color_str = color_str.trim();
                let mut parts = color_str.split_ascii_whitespace();
                let num = i32::from_str_radix(parts.next().unwrap(), 10).unwrap();
                let color_str = parts.next().unwrap();
                assert_eq!(parts.next(), None);
                rgb.insert(color_str.to_string(), num);
            }
            grabs.push(rgb);
        }
        Game { nr, grabs }
    }

    fn possible_with(&self, contents: &HashMap<String, i32>) -> bool {
        for grab in self.grabs.iter() {
            for (color, num) in grab.iter() {
                let x = contents.get(color).cloned().unwrap_or(0);
                if num > &x {
                    return false;
                }
            }
        }
        return true;
    }
}

pub fn run(input: &str) -> usize {
    let contents = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);
    let mut sum: i32 = 0;
    for l in input.lines() {
        let game = Game::parse(l);
        let possible = game.possible_with(&contents);
        println!("{possible} {game:?}");
        if possible {
            sum += game.nr
        }
    }
    sum as usize
}
