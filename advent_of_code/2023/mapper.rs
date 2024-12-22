// All those map tasks need a common lib.
// Copied out after day 11.

use itertools::Itertools;

// The raw map.
#[derive(Clone, PartialEq, Eq)]
pub struct Map<AuxT = ()> {
    map: Vec<Vec<char>>,
    default: char,
    aux: Vec<Vec<AuxT>>,
}

// A transformation.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]

enum TransformOp {
    DiagonalFlip,
    ClockwiseRot90,
    #[allow(dead_code)]
    CounterClockwiseRot90,
}

pub struct TransformType {
    transforms: Vec<TransformOp>,
}

// A Map View
pub struct Transform<'a, 'b, AuxT = ()> {
    map: &'a Map<AuxT>,
    tr: &'b TransformType,
}

// A Map Editor
pub struct TransformMut<'a, 'b, AuxT = ()> {
    map: &'a mut Map<AuxT>,
    tr: &'b TransformType,
}

// A point on the map. Purposefully not usize!
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Pos {
    r: isize,
    c: isize,
}

#[derive(Debug)]
pub struct PosIter {
    r: isize,
    c: isize,
    r_range: std::ops::Range<isize>,
    c_range: std::ops::Range<isize>,
}

pub const UP: Pos = Pos { r: -1, c: 0 };
pub const DOWN: Pos = Pos { r: 1, c: 0 };
pub const LEFT: Pos = Pos { r: 0, c: -1 };
pub const RIGHT: Pos = Pos { r: 0, c: 1 };
pub const DIRS: [Pos; 4] = [UP, DOWN, LEFT, RIGHT];
pub const ORIGO: Pos = Pos { r: 0, c: 0 };

// Common for Map and View
pub trait IView<AuxT>: std::ops::Index<Pos, Output = char> {
    fn row_column(&self) -> (isize, isize);
    fn get(&self, p: Pos) -> char;
    fn get_aux(&self, p: Pos) -> &AuxT;

    fn rows(&self) -> isize {
        self.row_column().0
    }

    fn row_range(&self) -> std::ops::Range<isize> {
        0..self.rows() as isize
    }

    fn columns(&self) -> isize {
        self.row_column().1
    }

    fn column_range(&self) -> std::ops::Range<isize> {
        0..self.columns() as isize
    }

    fn contains_pos(&self, p: Pos) -> bool {
        self.row_range().contains(&p.r) && self.column_range().contains(&p.c)
    }

    fn pos_iter(&self) -> PosIter {
        let r_range = self.row_range();
        let c_range = self.column_range();
        PosIter {
            r: r_range.start,
            c: c_range.start - 1,
            r_range,
            c_range,
        }
    }

    fn pos_usize(&self, p: Pos) -> Option<(usize, usize)> {
        if (0..self.rows()).contains(&p.r) {
            if (0..self.columns()).contains(&p.c) {
                return Some((p.r as usize, p.c as usize));
            }
        }
        return None;
    }

    fn dump(&self) {
        for r in self.row_range() {
            for c in self.column_range() {
                print!("{}", self.get(pos(r, c)));
            }
            println!("");
        }
    }
}

// Additional editor trait.
pub trait IViewMut<AuxT>: IView<AuxT> + std::ops::IndexMut<Pos, Output = char> {
    fn set(&mut self, p: Pos, ch: char);
    fn set_aux(&mut self, p: Pos, a: AuxT);
    fn mut_aux(&mut self, p: Pos) -> &mut AuxT;
}

pub fn pos(r: isize, c: isize) -> Pos {
    Pos { r, c }
}

impl Pos {
    #[inline]
    pub fn row(&self) -> isize {
        self.r
    }

    #[inline]
    pub fn column(&self) -> isize {
        self.c
    }

    // Returns the box containing these positions.
    // - top left corner
    // - bottom right corner
    pub fn boxing_range<'a, I>(i: I) -> (Pos, Pos)
    where
        I: Iterator<Item = &'a Pos>,
    {
        let mut minr = isize::MAX;
        let mut maxr = isize::MIN;
        let mut minc = isize::MAX;
        let mut maxc = isize::MIN;
        for p in i {
            use std::cmp::{max, min};
            minr = min(minr, p.r);
            maxr = max(maxr, p.r);
            minc = min(minc, p.c);
            maxc = max(maxc, p.c);
        }
        (pos(minr, minc), pos(maxr, maxc))
    }
}

impl Iterator for PosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.r_range.contains(&self.r) {
            return None;
        }
        self.c+=1;
        if !self.c_range.contains(&self.c) {
            self.c = self.c_range.start;
            self.r += 1;
        }
        if !self.r_range.contains(&self.r) {
            return None;
        }
        return Some(pos(self.r,self.c));
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            r: self.r + rhs.r,
            c: self.c + rhs.c,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.c += rhs.c;
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            r: self.r - rhs.r,
            c: self.c - rhs.c,
        }
    }
}

impl std::ops::SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.c -= rhs.c;
    }
}

impl std::ops::Mul<isize> for Pos {
    type Output = Pos;

    fn mul(self, m: isize) -> Self::Output {
        Pos {
            r: self.r * m,
            c: self.c * m,
        }
    }
}

impl std::ops::MulAssign<isize> for Pos {
    fn mul_assign(&mut self, m: isize) {
        self.r *= m;
        self.c *= m;
    }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, m: i32) -> Self::Output {
        Pos {
            r: self.r * m as isize,
            c: self.c * m as isize,
        }
    }
}

impl std::ops::MulAssign<i32> for Pos {
    fn mul_assign(&mut self, m: i32) {
        self.r *= m as isize;
        self.c *= m as isize;
    }
}

impl<AuxT> std::ops::Index<Pos> for Map<AuxT> {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        match self.pos_usize(index) {
            Some((r, c)) => &self.map[r][c],
            None => panic!("{index:?}"),
        }
    }
}
impl<AuxT> std::ops::IndexMut<Pos> for Map<AuxT> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        match self.pos_usize(index) {
            Some((r, c)) => &mut self.map[r][c],
            None => panic!("{index:?}"),
        }
    }
}

impl<AuxT> IView<AuxT> for Map<AuxT> {
    fn row_column(&self) -> (isize, isize) {
        (self.map.len() as isize, self.map[0].len() as isize)
    }

    fn get_aux(&self, p: Pos) -> &AuxT {
        match self.pos_usize(p) {
            Some((r, c)) => &self.aux[r][c],
            None => panic!("{p:?}"),
        }
    }

    fn get(&self, p: Pos) -> char {
        match self.pos_usize(p) {
            Some((r, c)) => self.map[r][c],
            None => self.default,
        }
    }

    fn rows(&self) -> isize {
        self.row_column().0
    }

    fn row_range(&self) -> std::ops::Range<isize> {
        0..self.rows() as isize
    }

    fn columns(&self) -> isize {
        self.row_column().1
    }

    fn column_range(&self) -> std::ops::Range<isize> {
        0..self.columns() as isize
    }

    fn contains_pos(&self, p: Pos) -> bool {
        self.row_range().contains(&p.r) && self.column_range().contains(&p.c)
    }

    fn pos_usize(&self, p: Pos) -> Option<(usize, usize)> {
        if (0..self.rows()).contains(&p.r) {
            if (0..self.columns()).contains(&p.c) {
                return Some((p.r as usize, p.c as usize));
            }
        }
        return None;
    }

    fn dump(&self) {
        for r in self.row_range() {
            for c in self.column_range() {
                print!("{}", self.get(pos(r, c)));
            }
            println!("");
        }
    }
}

impl<AuxT> IViewMut<AuxT> for Map<AuxT> {
    fn set(&mut self, p: Pos, ch: char) {
        let (r, c) = self.pos_usize(p).unwrap();
        self.map[r][c] = ch;
    }

    fn set_aux(&mut self, p: Pos, a: AuxT) {
        let (r, c) = self.pos_usize(p).unwrap();
        self.aux[r][c] = a;
    }

    fn mut_aux(&mut self, p: Pos) -> &mut AuxT {
        let (r, c) = self.pos_usize(p).unwrap();
        &mut self.aux[r][c]
    }
}

impl Map {
    pub fn empty(rows: isize, columns: isize) -> Map {
        Map {
            map: vec![vec!['.'; columns as usize]; rows as usize],
            default: '.',
            aux: vec![vec![(); columns as usize]; rows as usize],
        }
    }

    pub fn set_default(&mut self, c:char) {
        self.default = c;
    }
}

impl<'a> From<Vec<&'a str>> for Map {
    fn from(vec: Vec<&'a str>) -> Self {
        Map::from_iter(vec.iter().copied())
    }
}

impl<'a> FromIterator<&'a str> for Map {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut m: Map = Map {
            map: iter
                .into_iter()
                .map(|l| l.chars().collect_vec())
                .collect_vec(),
            default: '.',
            aux: vec![],
        };
        assert_ne!(m.map.len(), 0);
        let len = m.map[0].len();
        assert_ne!(len, 0);
        for r in 0..m.map.len() {
            assert_eq!(m.map[r].len(), len);
        }
        for r in &m.map {
            m.aux.push(vec![(); r.len()]);
        }
        m
    }
}

impl<AuxT> Map<AuxT> {
    pub fn with_aux<A2, FN>(self, f: FN) -> Map<A2>
    where
        FN: Fn(AuxT) -> A2,
    {
        Map {
            map: self.map,
            default: self.default,
            aux: self
                .aux
                .into_iter()
                .map(|r| r.into_iter().map(&f).collect_vec())
                .collect_vec(),
        }
    }

    fn set(&mut self, p: Pos, ch: char) {
        let (r, c) = self.pos_usize(p).unwrap();
        self.map[r][c] = ch;
    }

    pub fn view<'a, 'b>(&'a self, tr: &'b TransformType) -> Transform<'a, 'b, AuxT> {
        Transform { map: self, tr }
    }

    #[allow(dead_code)]
    pub fn view_mut<'a, 'b>(&'a mut self, tr: &'b TransformType) -> TransformMut<'a, 'b, AuxT> {
        TransformMut { map: self, tr }
    }
}

impl TransformType {
    fn row_column<AuxT>(&self, m: &Map<AuxT>) -> (isize, isize) {
        let (mut r, mut c) = m.row_column();
        for op in self.transforms.iter() {
            match op {
                TransformOp::DiagonalFlip => (r, c) = (c, r),
                TransformOp::ClockwiseRot90 => (r, c) = (c, r),
                TransformOp::CounterClockwiseRot90 => (r, c) = (c, r),
            }
        }
        (r, c)
    }

    fn remap<AuxT>(&self, m: &Map<AuxT>, p: Pos) -> Pos {
        let (mut r, mut c) = (p.r, p.c);
        for op in self.transforms.iter() {
            match op {
                TransformOp::DiagonalFlip => (r, c) = (c, r),
                TransformOp::ClockwiseRot90 => (r, c) = (m.columns() - c - 1, r),
                TransformOp::CounterClockwiseRot90 => (r, c) = (c, m.rows() - r - 1),
            }
        }
        pos(r, c)
    }

    fn add_transform_op(self, tt: TransformOp) -> TransformType {
        TransformType {
            transforms: {
                let mut transforms = self.transforms;
                transforms.push(tt);
                transforms
            },
            ..self
        }
    }

    pub fn flip_diagonally(self) -> TransformType {
        self.add_transform_op(TransformOp::DiagonalFlip)
    }

    pub fn clockwise_rot90(self) -> TransformType {
        self.add_transform_op(TransformOp::ClockwiseRot90)
    }

    #[allow(dead_code)]
    pub fn counter_clockwise_rot90(self) -> TransformType {
        self.add_transform_op(TransformOp::CounterClockwiseRot90)
    }
}

pub fn id() -> TransformType {
    TransformType { transforms: vec![] }
}

impl<AuxT> std::ops::Index<Pos> for Transform<'_, '_, AuxT> {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        self.map.index(self.tr.remap(self.map, index))
    }
}

impl<AuxT> IView<AuxT> for Transform<'_, '_, AuxT> {
    fn row_column(&self) -> (isize, isize) {
        let (r, c) = self.map.row_column();
        let p = self.tr.remap(self.map, pos(r, c));
        (p.r, p.c)
    }

    fn get(&self, p: Pos) -> char {
        self.map.get(self.tr.remap(self.map, p))
    }

    fn get_aux(&self, p: Pos) -> &AuxT {
        self.map.get_aux(self.tr.remap(self.map, p))
    }
}

impl<AuxT> std::ops::Index<Pos> for TransformMut<'_, '_, AuxT> {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        self.map.index(self.tr.remap(self.map, index))
    }
}

impl<AuxT> IView<AuxT> for TransformMut<'_, '_, AuxT> {
    fn row_column(&self) -> (isize, isize) {
        self.tr.row_column(self.map)
    }

    fn get(&self, p: Pos) -> char {
        self.map.get(self.tr.remap(self.map, p))
    }

    fn get_aux(&self, p: Pos) -> &AuxT {
        self.map.get_aux(self.tr.remap(self.map, p))
    }
}

impl<AuxT> std::ops::IndexMut<Pos> for TransformMut<'_, '_, AuxT> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        self.map.index_mut(self.tr.remap(self.map, index))
    }
}

impl<AuxT> IViewMut<AuxT> for TransformMut<'_, '_, AuxT> {
    fn set(&mut self, p: Pos, ch: char) {
        self.map.set(self.tr.remap(self.map, p), ch)
    }

    fn set_aux(&mut self, p: Pos, a: AuxT) {
        self.map.set_aux(self.tr.remap(self.map, p), a)
    }

    fn mut_aux(&mut self, p: Pos) -> &mut AuxT {
        self.map.mut_aux(self.tr.remap(self.map, p))
    }
}
