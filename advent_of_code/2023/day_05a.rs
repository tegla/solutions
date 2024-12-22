use itertools::Itertools;
use std::marker::PhantomData;

// This is an attemp to see how far I can drive a parser combinator impl
// with templated class compositions.
// Not very useful, at the end the main blocker became rust's insistence on not inferring
// return types.

trait ParseStr<T> {
    fn parse<'a>(&self, s: &'a str) -> Option<(T, &'a str)>;
}

struct ToI64 {
    radix: u32,
}

impl ParseStr<i64> for ToI64 {
    fn parse<'a>(&self, s: &'a str) -> Option<(i64, &'a str)> {
        //println!("ToI32 ");
        let mut result = None;
        for i in 1..s.len() + 1 {
            let consider = &s[0..i];
            //println!("ToI32: {consider}");
            if let Ok(n) = i64::from_str_radix(consider, self.radix) {
                result = Some((n, &s[i..s.len()]));
            } else {
                break;
            }
        }
        result
    }
}

fn to_i64() -> ToI64 {
    ToI64 { radix: 10 }
}

struct ConsumeString {
    s: String,
}

impl ParseStr<String> for ConsumeString {
    fn parse<'a>(&self, s: &'a str) -> Option<(String, &'a str)> {
        if s.starts_with(&self.s) {
            Some((self.s.clone(), &s[self.s.len()..]))
        } else {
            None
        }
    }
}

fn consume(s: &str) -> ConsumeString {
    ConsumeString { s: s.to_string() }
}

struct RepeatedStr<T, P: ParseStr<T>, C: ParseStr<()>> {
    parser: P,
    delimiter: C,
    _phantom: PhantomData<T>,
}

impl<T, P: ParseStr<T>, C: ParseStr<()>> ParseStr<Vec<T>> for RepeatedStr<T, P, C> {
    fn parse<'a>(&self, s: &'a str) -> Option<(Vec<T>, &'a str)> {
        let mut result: Vec<T> = Vec::new();
        let mut rest = s;
        //println!("'{rest}'");
        if let Some((t, r)) = self.parser.parse(rest) {
            result.push(t);
            rest = r;
        } else {
            // At least one must happen.
            return None;
        }

        loop {
            // println!("'{rest}'");
            if let Some((_, r1)) = self.delimiter.parse(rest) {
                if let Some((t, r2)) = self.parser.parse(r1) {
                    result.push(t);
                    rest = r2;
                    continue;
                }
            }
            break;
        }

        Some((result, rest))
    }
}

fn repeated_delim<T, P: ParseStr<T>, C: ParseStr<()>>(
    parser: P,
    delimiter: C,
) -> RepeatedStr<T, P, C> {
    RepeatedStr {
        delimiter,
        parser,
        _phantom: PhantomData,
    }
}

fn repeated_whitespace<T, P: ParseStr<T>>(parser: P) -> RepeatedStr<T, P, WhiteSpaceConsumer> {
    RepeatedStr {
        delimiter: WhiteSpaceConsumer { min: 1 },
        parser,
        _phantom: PhantomData,
    }
}

fn repeated<T, P: ParseStr<T>>(parser: P) -> RepeatedStr<T, P, Empty> {
    RepeatedStr {
        delimiter: Empty {},
        parser,
        _phantom: PhantomData,
    }
}

struct WhiteSpaceConsumer {
    min: i32,
}

impl ParseStr<()> for WhiteSpaceConsumer {
    fn parse<'a>(&self, s: &'a str) -> Option<((), &'a str)> {
        let mut tmp = s;
        loop {
            if let Some(c) = tmp.chars().next() {
                if c.is_whitespace() && !c.is_control() {
                    tmp = &tmp[1..];
                    continue;
                }
            }
            break;
        }
        if (s.len() - tmp.len()) >= self.min as usize {
            Some(((), tmp))
        } else {
            None
        }
    }
}

struct ConvertStr<I, O, F, P>
where
    F: Fn(I) -> O,
    P: ParseStr<I>,
{
    f: F,
    p: P,
    _phantom_i: PhantomData<I>,
    _phantom_o: PhantomData<O>,
}

impl<I, O, F: Fn(I) -> O, P: ParseStr<I>> ParseStr<O> for ConvertStr<I, O, F, P> {
    fn parse<'a>(&self, s: &'a str) -> Option<(O, &'a str)> {
        if let Some((i, s2)) = self.p.parse(s) {
            Some(((self.f)(i), s2))
        } else {
            None
        }
    }
}

fn mapto<I, O, F: Fn(I) -> O, P: ParseStr<I>>(f: F, p: P) -> ConvertStr<I, O, F, P> {
    ConvertStr {
        f,
        p,
        _phantom_i: PhantomData,
        _phantom_o: PhantomData,
    }
}

struct PairStr<T1, T2, P1: ParseStr<T1>, P2: ParseStr<T2>> {
    p1: P1,
    p2: P2,
    _t1: PhantomData<T1>,
    _t2: PhantomData<T2>,
}

impl<T1, T2, P1: ParseStr<T1>, P2: ParseStr<T2>> ParseStr<(T1, T2)> for PairStr<T1, T2, P1, P2> {
    fn parse<'a>(&self, s: &'a str) -> Option<((T1, T2), &'a str)> {
        if let Some((t1, s1)) = self.p1.parse(s) {
            if let Some((t2, s2)) = self.p2.parse(s1) {
                return Some(((t1, t2), s2));
            }
        }
        return None;
    }
}

fn pair<T1, T2, P1: ParseStr<T1>, P2: ParseStr<T2>>(p1: P1, p2: P2) -> PairStr<T1, T2, P1, P2> {
    PairStr {
        p1,
        p2,
        _t1: PhantomData,
        _t2: PhantomData,
    }
}

struct ConsumePrefix<T, P: ParseStr<T>> {
    prefix: String,
    p: P,
    _t: PhantomData<T>,
}

impl<T, P: ParseStr<T>> ParseStr<T> for ConsumePrefix<T, P> {
    fn parse<'a>(&self, s: &'a str) -> Option<(T, &'a str)> {
        if !s.starts_with(&self.prefix) {
            return None;
        }
        let s = &s[self.prefix.len()..];
        self.p.parse(s)
    }
}

fn consume_prefix<T, P: ParseStr<T>>(prefix: &str, p: P) -> ConsumePrefix<T, P> {
    ConsumePrefix {
        prefix: prefix.to_string(),
        p,
        _t: PhantomData,
    }
}

struct Line<T, P: ParseStr<T>> {
    p: P,
    _t: PhantomData<T>,
}

impl<T, P: ParseStr<T>> ParseStr<T> for Line<T, P> {
    fn parse<'a>(&self, s: &'a str) -> Option<(T, &'a str)> {
        if let Some((t, s1)) = self.p.parse(s) {
            if let Some(c) = s1.chars().next() {
                if c.is_control() {
                    return Some((t, &s1[1..]));
                }
            }
        }
        None
    }
}

fn line<T, P: ParseStr<T>>(p: P) -> Line<T, P> {
    Line { p, _t: PhantomData }
}

struct Empty {}

impl ParseStr<()> for Empty {
    fn parse<'a>(&self, s: &'a str) -> Option<((), &'a str)> {
        Some(((), s))
    }
}

fn empty() -> Empty {
    Empty {}
}

fn fullparse<T, P: ParseStr<T>>(p: P, s: &str) -> Option<T> {
    if let Some((t, s)) = p.parse(s) {
        if s.is_empty() {
            return Some(t);
        }
    }
    None
}

struct Word {}

impl ParseStr<String> for Word {
    fn parse<'a>(&self, s: &'a str) -> Option<(String, &'a str)> {
        let mut result = String::new();
        let mut s = s;
        loop {
            if let Some(c) = s.chars().next() {
                if c.is_alphabetic() {
                    result.push(c);
                    s = &s[1..];
                    continue;
                }
            }
            break;
        }
        if result.is_empty() {
            None
        } else {
            Some((result, s))
        }
    }
}

fn word() -> Word {
    Word {}
}

#[derive(Debug)]
struct RangeMap {
    d: i64,
    s: i64,
    l: i64,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Mapper {
    from: String,
    to: String,

    m: Vec<RangeMap>,
}

fn remap(ns: Vec<i64>, m: &Mapper) -> Vec<i64> {
    let mut result = Vec::new();
    for n in ns {
        let mut n2 = None;
        for m in &m.m {
            if (m.s..m.s + m.l).contains(&n) {
                n2 = Some(n - m.s + m.d);
                break;
            }
        }
        result.push(n2.unwrap_or(n));
    }
    result
}

pub fn run(input: &str) -> usize {
    let seeds_parser = line(consume_prefix("seeds: ", repeated_whitespace(to_i64())));
    let map_parser = pair(
        line(pair(
            pair(word(), consume_prefix("-to-", word())),
            consume(" map:"),
        )),
        repeated(mapto(
            |v| {
                let (d, s, l) = v.into_iter().collect_tuple().unwrap();
                RangeMap { d, s, l }
            },
            line(repeated_whitespace(to_i64())),
        )),
    );
    let map_parser = mapto(|(((from, to), _), m)| Mapper { from, to, m }, map_parser);
    let full_parser = pair(
        pair(seeds_parser, line(empty())),
        repeated_delim(map_parser, line(empty())),
    );
    let (seeds, mappers) = fullparse(full_parser, input).unwrap();
    let (mut seeds, _) = seeds;
    println!("{seeds:?}");
    for m in mappers.iter() {
        println!("{m:?}");
        seeds = remap(seeds, m);
        println!("{seeds:?}");
    }
    *seeds.iter().min().unwrap() as usize
}
