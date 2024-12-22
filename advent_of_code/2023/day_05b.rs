use crate::parser::*;
use std::collections::BTreeSet;

impl SR {
    fn parse() -> impl Fn(&mut &str) -> Option<SR> {
        try_parse(|s| {
            let d = i64()(s)?;
            ws()(s)?;
            let s2 = i64()(s)?;
            ws()(s)?;
            let l = i64()(s)?;
            Some(SR {
                shift: d - s2,
                range: R {
                    start: s2,
                    end: s2 + l,
                },
            })
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Mapper {
    from: String,
    to: String,

    srs: Vec<SR>,
}

impl Mapper {
    fn parse() -> impl Fn(&mut &str) -> Option<Mapper> {
        try_parse(|s| {
            let from = word()(s)?;
            string_match("-to-")(s)?;
            let to = word()(s)?;
            line(string_match(" map:"))(s)?;
            let srs = rep1(line(SR::parse()))(s)?;
            Some(Mapper { from, to, srs })
        })
    }
}

fn parse() -> impl Fn(&mut &str) -> Option<(Vec<R>, Vec<Mapper>)> {
    try_parse(|s| {
        let seed = tup2(first(i64(), ws()), i64());
        let seed = transform(seed, |(start, len)| R {
            start,
            end: start + len,
        });
        let seeds = line(second(string_match("seeds: "), rep_del(seed, ws())))(s)?;
        line(empty())(s)?;
        let ms = rep_del(Mapper::parse(), line(empty()))(s)?;

        Some((seeds, ms))
    })
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct R {
    start: i64,
    end: i64,
}

impl R {
    fn contains(&self, o: Self) -> bool {
        self.start <= o.start && self.end >= o.end
    }

    fn cuts(rs: &[Self]) -> Vec<R> {
        let mut cs = BTreeSet::new();
        for r in rs {
            cs.insert(r.start);
            cs.insert(r.end);
        }

        cs.iter()
            .copied()
            .zip(cs.iter().copied().skip(1))
            .map(|(start, end)| R { start, end })
            .collect()
    }

    fn stays_moves(&self, sr: SR) -> (Vec<R>, Vec<R>) {
        let cs = R::cuts(&[*self, sr.range]);
        let mut stays = Vec::new();
        let mut moves = Vec::new();
        for c in cs {
            if !self.contains(c) {
                continue;
            }
            if sr.range.contains(c) {
                moves.push(c.shift(sr.shift))
            } else {
                stays.push(c);
            }
        }
        (stays, moves)
    }

    fn moves(&self, srs: &[SR]) -> Vec<R> {
        let mut result = Vec::new();
        let mut all_stays = vec![*self];
        for sr in srs {
            let mut all_stays_tmp: Vec<R> = Vec::new();
            all_stays_tmp.append(&mut all_stays);
            assert!(all_stays.is_empty());
            for s in all_stays_tmp {
                let (mut stays, mut moves) = s.stays_moves(*sr);
                result.append(&mut moves);
                all_stays.append(&mut stays)
            }
        }
        result.append(&mut all_stays);
        result
    }

    fn shift(&self, s: i64) -> R {
        R {
            start: self.start + s,
            end: self.end + s,
        }
    }

    fn move_all(rs: &[R], srs: &[SR]) -> Vec<R> {
        rs.iter().map(|r| r.moves(srs)).flatten().collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct SR {
    shift: i64,
    range: R,
}

pub fn run(input: &str) -> usize {
    let (seeds, mappers) = full_parse(input, parse());

    let mut mins = Vec::new();
    for seed in seeds.iter() {
        let mut rs = vec![*seed];
        for mr in mappers.iter() {
            rs = R::move_all(&rs, &mr.srs);
        }
        let min = rs.iter().map(|r| r.start).min().unwrap();
        println!("seed {:?} -> {}", *seed, min);
        mins.push(min);
    }

    *mins.iter().min().unwrap() as usize
}
