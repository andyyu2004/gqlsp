pub use maplit::hashmap;

use std::collections::{HashMap, HashSet};

use gqls_syntax::{Point, Position};
use vfs::FileId;

#[macro_export]
macro_rules! fixture {
    ( $($file:literal => $text:expr)* ) => {{
        let mut vfs = vfs::Vfs::default();
        $crate::Fixture::new(
            $crate::hashmap! {
                $(vfs.intern($file) => $crate::FixtureFile::parse(&$text)),*
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

    pub fn all_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.files
            .iter()
            .flat_map(|(&path, file)| std::iter::repeat(path).zip(file.points.iter().copied()))
            .map(|(file, point)| Position { file, point })
    }

    pub fn all_ranges(&self) -> impl Iterator<Item = (FileId, std::ops::Range<Point>)> + '_ {
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
/// - delimited ranges `(delimited above by `{` and below by `}` )`
///   This ranges from the start of the following line of `{` to the end of the the preceding line of `}`
pub struct FixtureFile {
    pub points: Vec<Point>,
    pub ranges: Vec<std::ops::Range<Point>>,
    pub text: String,
}

impl FixtureFile {
    pub fn parse(fixture: &str) -> Self {
        let mut points = vec![];
        let mut ranges = vec![];
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
                    ranges.push(start..Point { row: row - 1, column })
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

        Self { points, ranges, text: fixture.replace('$', " ") }
    }
}

#[cfg(test)]
mod tests;
