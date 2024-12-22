use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools;

// Note: this code is very complicated, because I wrote a lot of different
// validation codes to figure out why my code doesn't give an acceptable answer.
// It took a while that it was working fine; but I gave back the bricks that
// _cannot_ be disintegrated.

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

    for b in bricks.iter() {
        println!("{b:?}");
    }

    let max_x = bricks.iter().map(|b| b.b.x).max().unwrap();
    let max_y = bricks.iter().map(|b| b.b.y).max().unwrap();

    // Validation, this keeps bricks in falling order for every point.
    for x in 0..=max_x + 1 {
        for y in 0..max_y + 1 {
            let mut current_top: Option<Brick> = None;
            for i in 0..bricks.len() {
                let brick = bricks[i];
                if (brick.a.x..=brick.b.x).contains(&(x as i32))
                    && (brick.a.y..=brick.b.y).contains(&(y as i32))
                {
                    if let Some(prev) = current_top {
                        assert!(prev.b.z < brick.a.z);
                    }
                    current_top = Some(brick);
                }
            }
        }
    }

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

    // Validate the final positions.
    let mut supported_by: BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
    for x in 0..=max_x + 1 {
        for y in 0..max_y + 1 {
            let mut current_height = 0;
            let mut current_top: Option<usize> = None;
            for i in 0..bricks.len() {
                let brick = bricks[i];
                if (brick.a.x..=brick.b.x).contains(&(x as i32))
                    && (brick.a.y..=brick.b.y).contains(&(y as i32))
                {
                    if let Some(prev) = current_top {
                        assert!(bricks[prev].b.z < brick.a.z);
                        assert!(
                            current_height < settled_z[i],
                            "x={x} y={y} i={i}, {current_height}<{}",
                            settled_z[i]
                        );
                        if current_height + 1 == settled_z[i] {
                            supported_by.entry(i).or_default().insert(prev);
                        }
                    }
                    current_top = Some(i);
                    current_height = settled_z[i] + brick.b.z - brick.a.z;
                }
            }
        }
    }

    let mut critical_bricks2: BTreeSet<usize> = BTreeSet::new();
    for (_k, v) in supported_by.iter() {
        // println!("{_k} supported_by: {:?}", v);
        if v.len() < 2 {
            critical_bricks2.extend(v.iter());
        }
    }
    let critical_bricks2 = critical_bricks2.iter().copied().collect_vec();

    println!("{critical_bricks2:?}");

    let critical_bricks = critical_bricks
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|x| x.0)
        .collect_vec();
    println!("{critical_bricks:?}");
    assert_eq!(critical_bricks, critical_bricks2);

    // Yet an other validation logic. I'm getting desperate.
    // Does every brick lay on an other brick?
    for i in 0..bricks.len() {
        let bottom_i = settled_z[i];
        let mut under = None;
        for j in 0..bricks.len() {
            if i == j {
                continue;
            }
            if !common_y_column(&bricks[i], &bricks[j]) {
                continue;
            }
            let top_j = settled_z[j] + bricks[j].b.z - bricks[j].a.z;
            if top_j < bottom_i {
                if let Some(u) = under {
                    if top_j > u {
                        under = Some(top_j)
                    }
                } else {
                    under = Some(top_j);
                }
            }
        }
        if let Some(u) = under {
            assert_eq!(u + 1, bottom_i);
        } else {
            assert_eq!(bottom_i, 1, "i={i}");
        }
    }

    // 934: too high
    bricks.len() -    critical_bricks.len()
}
