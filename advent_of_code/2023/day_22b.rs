use std::collections::BTreeSet;
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Brick {
    a: Point,
    b: Point,
}

fn common_y_column(one: &Brick, other: &Brick) -> bool {
    use std::cmp::{max, min};
    let common_start_x = max(one.a.x, other.a.x);
    let common_end_x = min(one.b.x, other.b.x);
    if common_start_x > common_end_x {
        return false;
    }
    let common_start_y = max(one.a.y, other.a.y);
    let common_end_y = min(one.b.y, other.b.y);
    if common_start_y > common_end_y {
        return false;
    }
    return true;
}

impl std::cmp::PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        if !common_y_column(self, other) {
            return None;
        }
        if self.b.z < other.a.z {
            return Some(Ordering::Less);
        } else if self.a.z > other.b.z {
            return Some(Ordering::Greater);
        } else {
            panic!("{self:?} {other:?}")
        }
    }
}

pub fn run(input: &str) -> usize {
    let mut bricks = vec![];
    for l in input.lines() {
        let (a, b) = l.split_once('~').expect(l);
        let (x, y, z) = a
            .split(',')
            .map(|n| i32::from_str_radix(n, 10).expect(n))
            .collect_tuple()
            .expect(a);
        let a = Point { x, y, z };
        let (x, y, z) = b
            .split(',')
            .map(|n| i32::from_str_radix(n, 10).expect(n))
            .collect_tuple()
            .expect(b);
        let b = Point { x, y, z };
        assert!(a.x <= b.x);
        assert!(a.y <= b.y);
        assert!(a.z <= b.z);
        bricks.push(Brick { a, b });
    }

    // Sort by topological ordering.
    for i in 0..bricks.len() {
        'loop_j: for j in i..bricks.len() {
            for k in i..bricks.len() {
                if j == k {
                    continue;
                }
                if bricks[k] < bricks[j] {
                    continue 'loop_j;
                }
            }
            (bricks[i], bricks[j]) = (bricks[j], bricks[i]);
        }
    }
    let bricks = bricks;

    let max_x = bricks.iter().map(|b| b.b.x).max().unwrap();
    let max_y = bricks.iter().map(|b| b.b.y).max().unwrap();

    // point is occupied by brick n, at height h
    let mut height: Vec<Vec<i32>> = vec![vec![0; max_y as usize + 1]; max_x as usize + 1];
    let mut top_brick: Vec<Vec<usize>> =
        vec![vec![usize::MAX; max_y as usize + 1]; max_x as usize + 1];
    let mut settled_z = vec![0; bricks.len()];

    let mut critical_bricks = vec![false; bricks.len()];

    for (i, brick) in bricks.iter().enumerate() {
        println!("{i} {brick:?}");
        for x in 0..top_brick.len() {
            for y in 0..top_brick[x].len() {
                if (brick.a.x..=brick.b.x).contains(&(x as i32))
                    && (brick.a.y..=brick.b.y).contains(&(y as i32))
                {
                    print!("    *");
                } else {
                    print!("    .");
                }
            }
            println!("");
        }
        println!("");
        let mut supporting_bricks = HashSet::new();
        let mut supporting_height = -1;
        for x in brick.a.x..=brick.b.x {
            for y in brick.a.y..=brick.b.y {
                let b2 = top_brick[x as usize][y as usize];
                let h2 = height[x as usize][y as usize];
                if h2 > supporting_height {
                    supporting_height = h2;
                    supporting_bricks.clear();
                }
                if h2 == supporting_height && b2 != usize::MAX {
                    supporting_bricks.insert(b2);
                }
            }
        }
        for x in brick.a.x..=brick.b.x {
            for y in brick.a.y..=brick.b.y {
                top_brick[x as usize][y as usize] = i;
                height[x as usize][y as usize] = supporting_height + brick.b.z - brick.a.z + 1;
            }
        }

        settled_z[i] = supporting_height + 1;

        for x in 0..height.len() {
            for y in 0..height[x].len() {
                if height[x][y] == 0 {
                    print!("    .");
                } else {
                    print!(" {:4}", height[x][y])
                }
            }
            println!("");
        }
        println!("");
        for x in 0..top_brick.len() {
            for y in 0..top_brick[x].len() {
                if top_brick[x][y] == usize::MAX {
                    print!("    .");
                } else {
                    print!(" {:4}", top_brick[x][y])
                }
            }
            println!("");
        }
        println!("h:{supporting_height} supp:{:?}", supporting_bricks);
        if supporting_bricks.len() == 1 {
            critical_bricks[*supporting_bricks.iter().next().unwrap() as usize] = true;
        }
        println!("");
    }

    let mut supported_by = vec![BTreeSet::new(); bricks.len()];
    for i in 0..bricks.len() {
        let bottom_i = settled_z[i];
        for j in 0..bricks.len() {
            if i == j {
                continue;
            }
            if !common_y_column(&bricks[i], &bricks[j]) {
                continue;
            }
            let top_j = settled_z[j] + bricks[j].b.z - bricks[j].a.z;
            if top_j + 1 == bottom_i {
                supported_by[i].insert(j);
            }
        }
    }

    let mut transitive_supports = vec![BTreeSet::new(); bricks.len()];
    for i in 0..bricks.len() {
        let mut removed = BTreeSet::new();
        removed.insert(i);
        for j in i + 1..bricks.len() {
            if !supported_by[j].is_empty() && removed.is_superset(&supported_by[j]) {
                removed.insert(j);
            }
        }
        removed.remove(&i);
        transitive_supports[i] = removed;
    }

    let transitive_supports = transitive_supports.iter().map(|x| x.len()).collect_vec();
    println!("{transitive_supports:?}");

    transitive_supports.iter().sum()
}
