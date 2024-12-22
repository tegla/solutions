
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiRange {
    rs: Vec<(isize, isize)>,
}

impl MultiRange {
    pub fn count(&self) -> usize {
        let mut c = 0;
        for (l, r) in self.rs.iter().copied() {
            c += (r - l) as usize;
        }
        c
    }

    pub fn is_empty(&self) -> bool {
        self.rs.is_empty()
    }

    pub fn empty() -> Self {
        MultiRange { rs: vec![] }
    }

    pub fn is_full(&self) -> bool {
        self.rs.len() == 1 && self.rs[0] == (isize::MIN, isize::MAX)
    }

    pub fn full() -> MultiRange {
        MultiRange::from(isize::MIN..isize::MAX)
    }

    pub fn from(range: std::ops::Range<isize>) -> Self {
        MultiRange {
            rs: vec![(range.start, range.end)],
        }
    }

    fn merge_with_and(rs1: &Vec<(isize, isize)>, rs2: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
        if rs1.is_empty() || rs2.is_empty() {
            return vec![];
        }
        let mut rs = Vec::new();
        let mut i1 = rs1.iter().copied();
        let mut i2 = rs2.iter().copied();
        let mut r1 = i1.next().unwrap();
        let mut r2 = i2.next().unwrap();
        loop {
            // println!("{r1:?}  {r2:?}");
            if r1.0 != r2.0 {
                let max = std::cmp::max(r1.0, r2.0);
                r1.0 = max;
                r2.0 = max;
            }
            if r1.0 >= r1.1 {
                if let Some(r) = i1.next() {
                    r1 = r;
                    continue;
                } else {
                    break;
                }
            }
            if r2.0 >= r2.1 {
                if let Some(r) = i2.next() {
                    r2 = r;
                    continue;
                } else {
                    break;
                }
            }
            let e = std::cmp::min(r1.1, r2.1);
            rs.push((r1.0, e));
            r1.0 = e;
            r2.0 = e;
        }
        rs
    }
}

impl std::ops::BitAnd<MultiRange> for MultiRange {
    type Output = MultiRange;

    fn bitand(self, rhs: Self) -> Self::Output {
        MultiRange {
            rs: MultiRange::merge_with_and(&self.rs, &rhs.rs),
        }
    }
}

impl std::ops::BitAndAssign for MultiRange {
    fn bitand_assign(&mut self, rhs: Self) {
        self.rs = MultiRange::merge_with_and(&self.rs, &rhs.rs);
    }
}

impl std::ops::Not for MultiRange {
    type Output = MultiRange;

    fn not(self) -> Self::Output {
        if self.is_empty() {
            return MultiRange::full();
        }
        if self.is_full() {
            return MultiRange::empty();
        }

        let mut rs = vec![];

        let mut i = self.rs.iter().copied();
        let (first, mut last) = i.next().unwrap();
        if first != isize::MIN {
            rs.push((isize::MIN, first));
        }

        while let Some((s, e)) = i.next() {
            rs.push((last, s));
            last = e;
        }

        if last != isize::MAX {
            rs.push((last, isize::MAX));
        }

        MultiRange { rs }
    }
}

impl std::ops::BitOr for MultiRange {
    type Output = MultiRange;

    fn bitor(self, rhs: Self) -> Self::Output {
        // we should probably write a custom code here
        !(!self & !rhs)
    }
}

impl std::ops::BitOrAssign for MultiRange {
    fn bitor_assign(&mut self, rhs: Self) {
        // we should probably write a custom code here
        *self = self.clone() | rhs
    }
}
