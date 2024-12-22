// Very limited parser combinator for fast use.

pub fn try_parse<T, P: Fn(&mut &str) -> Option<T>>(p: P) -> impl Fn(&mut &str) -> Option<T> {
    move |s| {
        let saved = *s;
        match p(s) {
            Some(t) => Some(t),
            None => {
                *s = saved;
                None
            }
        }
    }
}

fn ch() -> impl Fn(&mut &str) -> Option<char> {
    |s| {
        if let Some(c) = (*s).chars().next() {
            *s = &(*s)[1..];
            Some(c)
        } else {
            None
        }
    }
}

fn char_filtered<FN: Fn(char) -> bool>(f: FN) -> impl Fn(&mut &str) -> Option<char> {
    try_parse(move |s| ch()(s).filter(|c| f(*c)))
}

pub fn char_in(chars:&str) -> impl Fn(&mut &str) -> Option<char> {
    let chars = chars.to_string();
    let f = move |c:char| {chars.contains(c)};
    char_filtered(f)
}

pub fn alphabetic() -> impl Fn(&mut &str) -> Option<char> {
    char_filtered(|c| c.is_alphabetic())
}

pub fn is_control() -> impl Fn(&mut &str) -> Option<char> {
    char_filtered(|c| c.is_control())
}

pub fn rep0<T, P: Fn(&mut &str) -> Option<T>>(p: P) -> impl Fn(&mut &str) -> Option<Vec<T>> {
    move |s| {
        let mut result = Vec::new();
        loop {
            match try_parse(&p)(s) {
                Some(t) => result.push(t),
                None => break,
            }
        }
        Some(result)
    }
}

pub fn rep1<T, P: Fn(&mut &str) -> Option<T>>(p: P) -> impl Fn(&mut &str) -> Option<Vec<T>> {
    try_parse(move |s| {
        let mut result = vec![p(s)?];
        loop {
            match try_parse(&p)(s) {
                Some(t) => result.push(t),
                None => break,
            }
        }
        Some(result)
    })
}

pub fn transform<I, O, P: Fn(&mut &str) -> Option<I>, T: Fn(I) -> O>(
    p: P,
    t: T,
) -> impl Fn(&mut &str) -> Option<O> {
    try_parse(move |s| Some(t(p(s)?)))
}

pub fn ws() -> impl Fn(&mut &str) -> Option<()> {
    transform(
        rep1(char_filtered(|c| c.is_whitespace() && !c.is_control())),
        |_| (),
    )
}

pub fn rep_del<T, D, PT: Fn(&mut &str) -> Option<T>, PD: Fn(&mut &str) -> Option<D>>(
    pt: PT,
    pd: PD,
) -> impl Fn(&mut &str) -> Option<Vec<T>> {
    try_parse(move |s| {
        let mut result = vec![pt(s)?];
        result.extend(rep0(second(&pd, &pt))(s)?);
        Some(result)
    })
}

pub fn tup2<T1, T2, P1: Fn(&mut &str) -> Option<T1>, P2: Fn(&mut &str) -> Option<T2>>(
    p1: P1,
    p2: P2,
) -> impl Fn(&mut &str) -> Option<(T1, T2)> {
    try_parse(move |s| Some((p1(s)?, p2(s)?)))
}

pub fn first<T1, T2, P1: Fn(&mut &str) -> Option<T1>, P2: Fn(&mut &str) -> Option<T2>>(
    p1: P1,
    p2: P2,
) -> impl Fn(&mut &str) -> Option<T1> {
    transform(tup2(p1, p2), |(t, _)| t)
}

pub fn second<T1, T2, P1: Fn(&mut &str) -> Option<T1>, P2: Fn(&mut &str) -> Option<T2>>(
    p1: P1,
    p2: P2,
) -> impl Fn(&mut &str) -> Option<T2> {
    transform(tup2(p1, p2), |(_, t)| t)
}

pub fn line<T, P: Fn(&mut &str) -> Option<T>>(p: P) -> impl Fn(&mut &str) -> Option<T> {
    first(p, is_control())
}

pub fn word() -> impl Fn(&mut &str) -> Option<String> {
    transform(rep1(alphabetic()), |vc| String::from_iter(vc.iter()))
}

fn max_accepted<T, P: Fn(&str) -> Option<T>>(min: usize, p: P) -> impl Fn(&mut &str) -> Option<T> {
    try_parse(move |s| {
        let start = *s;
        let mut result = None;
        for i in 0..start.len() + 1 {
            if let Some(t) = p(&start[0..i]) {
                result = Some(t);
                *s = &start[i..];
            } else {
                if i >= min {
                    break;
                }
            }
        }
        result
    })
}

#[allow(dead_code)]
pub fn i32r(radix: u32) -> impl Fn(&mut &str) -> Option<i32> {
    max_accepted(2, move |s| i32::from_str_radix(s, radix).ok())
}

#[allow(dead_code)]
pub fn i32() -> impl Fn(&mut &str) -> Option<i32> {
    i32r(10)
}

pub fn i64r(radix: u32) -> impl Fn(&mut &str) -> Option<i64> {
    max_accepted(2, move |s| i64::from_str_radix(s, radix).ok())
}

pub fn i64() -> impl Fn(&mut &str) -> Option<i64> {
    i64r(10)
}

pub fn string_match(m: &str) -> impl Fn(&mut &str) -> Option<String> {
    let m = m.to_string();
    move |s| {
        if s.starts_with(&m) {
            *s = &(*s)[m.len()..];
            Some(m.clone())
        } else {
            None
        }
    }
}

pub fn empty() -> impl Fn(&mut &str) -> Option<()> {
    |_| Some(())
}

pub fn full_parse<T, P: Fn(&mut &str) -> Option<T>>(s: &str, p: P) -> T {
    let mut s = s;
    let t = p(&mut s).unwrap();
    assert_eq!(s, "");
    t
}
