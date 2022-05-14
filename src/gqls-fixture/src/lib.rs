#![deny(rust_2018_idioms)]

pub use maplit::hashmap;

use std::collections::{HashMap, HashSet};

use gqls_syntax::{Point, Position};
use vfs::FileId;

#[macro_export]
macro_rules! fixture_file {
    ( $text:expr ) => {{ $crate::FixtureFile::parse(&$text) }};
}

#[macro_export]
macro_rules! fixture {
    ( $($file:literal => $text:expr)* ) => {{
        let mut vfs = vfs::Vfs::default();
        $crate::Fixture::new(
            $crate::hashmap! {
                $(vfs.intern($file) => $crate::fixture_file!($text)),*
            }
        )
    }};
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
    files: HashMap<FileId, FixtureFile>,
}

impl Fixture {
    pub fn new(files: HashMap<FileId, FixtureFile>) -> Self {
        Self { files }
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.files
            .iter()
            .flat_map(|(&path, file)| std::iter::repeat(path).zip(file.points.iter().copied()))
            .map(|(file, point)| Position { file, point })
    }

    pub fn annotations(
        &self,
    ) -> impl Iterator<Item = (FileId, std::slice::Iter<'_, Annotation>)> + '_ {
        self.files.iter().map(|(&path, file)| (path, file.annotations.iter()))
    }

    pub fn ranges(&self) -> impl Iterator<Item = (FileId, std::ops::Range<Point>)> + '_ {
        self.files
            .iter()
            .flat_map(|(&path, file)| std::iter::repeat(path).zip(file.ranges.iter().cloned()))
    }

    pub fn fileset(&self) -> HashSet<FileId> {
        self.files.keys().copied().collect()
    }

    pub fn files(&self) -> &HashMap<FileId, FixtureFile> {
        &self.files
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A fixture file supports three forms of annotations:
/// - points (`^`) (if ^ points to a range, then it is shifted a further one up)
/// - inline ranges (`...`)
///   - if an inline range is immediately followed by a open paren  `(`, then it is treated as an annotation up to the closing paren
/// - delimited ranges `(delimited above by `{` and below by `}` )`
///   This ranges from the start of the following line of `{` to the end of the the preceding line of `}`
pub struct FixtureFile {
    pub points: Vec<Point>,
    pub ranges: Vec<std::ops::Range<Point>>,
    pub text: String,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub range: std::ops::Range<Point>,
    pub text: String,
}

impl FixtureFile {
    pub fn parse(fixture: &str) -> Self {
        let mut points = vec![];
        let mut ranges = vec![];
        let mut annotations = vec![];
        let mut stack = vec![];

        for (row, (line, prev_line)) in
            fixture.lines().zip(std::iter::once("").chain(fixture.lines())).enumerate()
        {
            let mut range_start = None;
            for (column, char) in line.char_indices() {
                if char == '$' {
                    points.push(Point { row, column });
                }

                if !line.trim_start().starts_with('#') {
                    continue;
                }

                if char == '{' {
                    stack.push(Point { row: row + 1, column: 0 });
                } else if char == '}' {
                    let start = stack.pop().expect("unmatched `}`");
                    let end = Point { row: row - 1, column: prev_line.len() };
                    ranges.push(start..end);
                }

                if char == '.' {
                    if row == 0 {
                        panic!("cannot contain `.` in the first line");
                    }
                    if range_start.is_none() {
                        range_start = Some(Point { row: row - 1, column });
                    }
                } else if let Some(start) = range_start.take() {
                    let range = start..Point { row: row - 1, column };
                    if char == '(' {
                        let (text, _) = line[column + 1..]
                            .split_once(')')
                            .expect("missing closing delimiter for annotation");
                        assert!(!text.is_empty());
                        annotations.push(Annotation { range, text: text.to_owned() });
                    } else {
                        ranges.push(range)
                    }
                }

                if char == '^' {
                    if row == 0 {
                        panic!("cannot contain `^` in the first line");
                    }
                    let mut point = Point { row: row - 2, column };
                    if !ranges.iter().any(|range| range.contains(&point)) {
                        point.row += 1;
                    }
                    points.push(point);
                }
            }

            if let Some(start) = range_start {
                ranges.push(start..Point { row: row - 1, column: line.len() });
            }
        }

        Self { points, ranges, annotations, text: fixture.replace('$', " ") }
    }
}

#[cfg(test)]
mod tests;
