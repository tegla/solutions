use core::iter::Iterator;
use std::collections::HashSet;
use std::fmt;
use std::ops::Range;

struct Schema {
    map: Vec<Vec<char>>,
}

#[derive(Clone, Copy)]
struct Pos<'a> {
    schema: &'a Schema,
    row: i32,
    column: i32,
}

#[derive(Clone, Copy)]
struct Number<'a> {
    pos: Pos<'a>,
    length: i32,
    value: i32,
}

struct SchemaIter<'a> {
    pos: Pos<'a>,
}

impl Schema {
    fn parse(lines: Vec<&str>) -> Schema {
        Schema {
            map: lines.iter().map(|l| l.chars().collect()).collect(),
        }
    }

    fn row_range(&self) -> Range<i32> {
        0..self.map.len() as i32
    }

    fn column_range(&self) -> Range<i32> {
        0..self.map.get(0).unwrap().len() as i32
    }

    fn pos(&self, row: i32, column: i32) -> Pos {
        Pos {
            schema: self,
            row,
            column,
        }
    }

    fn iter<'a>(&'a self) -> SchemaIter<'a> {
        SchemaIter {
            pos: self.pos(0, -1),
        }
    }

    fn numbers(&self) -> Vec<Number> {
        let mut result = Vec::new();
        let mut iter = self.iter();
        while let Some(start) = iter.next() {
            let mut pos = *start;
            if !pos.get().is_numeric() {
                continue;
            }
            pos = pos.left();
            if pos.get().is_numeric() {
                continue;
            }
            let mut value = 0;
            let mut length = 0;
            pos = pos.right();
            while let Some(d) = pos.get().to_digit(10) {
                value *= 10;
                value += d as i32;
                length += 1;
                pos = pos.right();
            }

            let number = Number {
                pos: *start,
                value,
                length,
            };
            result.push(number);
        }
        result
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for mapline in self.map.iter() {
            for c in mapline.iter() {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

impl fmt::Display for Number<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}={}", self.pos, self.length, self.value)
    }
}

impl fmt::Display for Pos<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}.{:02}({})", self.row, self.column, self.get())
    }
}

impl Pos<'_> {
    fn get(&self) -> char {
        if !self.schema.row_range().contains(&self.row) {
            return '.';
        }
        if !self.schema.column_range().contains(&self.column) {
            return '.';
        }
        return self
            .schema
            .map
            .get(self.row as usize)
            .unwrap()
            .get(self.column as usize)
            .unwrap()
            .clone();
    }
    fn surrounding<'a>(&'a self) -> [Pos<'a>; 8] {
        [
            self.up(),
            self.up().left(),
            self.up().right(),
            self.left(),
            self.right(),
            self.down(),
            self.down().left(),
            self.down().right(),
        ]
    }
}

impl<'a> Pos<'a> {
    fn left(self) -> Pos<'a> {
        Pos {
            column: self.column - 1,
            ..self
        }
    }
    fn right(self) -> Pos<'a> {
        Pos {
            column: self.column + 1,
            ..self
        }
    }
    fn up(self) -> Pos<'a> {
        Pos {
            row: self.row - 1,
            ..self
        }
    }
    fn down(self) -> Pos<'a> {
        Pos {
            row: self.row + 1,
            ..self
        }
    }
}

impl<'a> SchemaIter<'a> {
    fn next(&mut self) -> Option<&Pos<'a>> {
        self.pos.column += 1;
        if !self.pos.schema.column_range().contains(&self.pos.column) {
            self.pos.column = 0;
            self.pos.row += 1;
        }
        if self.pos.schema.row_range().contains(&self.pos.row) {
            Some(&self.pos)
        } else {
            None
        }
    }
}

pub fn run(input: &str) -> usize {
    let schema = Schema::parse(input.lines().collect());
    let mut sum = 0;
    for number in schema.numbers() {
        let mut surrounding_chars = HashSet::new();
        let mut p = number.pos;
        for _ in 0..number.length {
            surrounding_chars.extend(p.surrounding().map(|p| p.get()));
            p = p.right()
        }
        let has_symbol = !surrounding_chars
            .iter()
            .all(|c| *c == '.' || c.is_numeric());
        println!("{number} {:?} {has_symbol} ", surrounding_chars);
        if has_symbol {
            sum += number.value
        }
    }
    sum as usize
}
