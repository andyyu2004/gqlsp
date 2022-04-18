pub use maplit::hashmap;

use std::collections::{HashMap, HashSet};

use gqls_parse::Point;
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

    pub fn all_points(&self) -> impl Iterator<Item = (FileId, Point)> + '_ {
        self.files
            .iter()
            .flat_map(|(&path, file)| std::iter::repeat(path).zip(file.points.iter().copied()))
    }

    pub fn all_ranges(&self) -> impl Iterator<Item = (FileId, std::ops::Range<Point>)> + '_ {
        self.files
            .iter()
            .flat_map(|(&path, file)| std::iter::repeat(path).zip(file.ranges.iter().cloned()))
    }

    pub fn sole_point(&self) -> (FileId, Point) {
        let mut sole_point = None;
        for (&file, file_fixture) in self.files() {
            if let Some(point) = file_fixture.sole_point() {
                if sole_point.is_some() {
                    panic!("multiple points in fixture");
                }
                sole_point = Some((file, point));
            }
        }
        sole_point.expect("no point in fixture")
    }

    pub fn sole_range(&self) -> (FileId, std::ops::Range<Point>) {
        let mut sole_range = None;
        for (&file, file_fixture) in self.files() {
            if let Some(range) = file_fixture.sole_range() {
                if sole_range.is_some() {
                    panic!("multiple ranges in fixture");
                }
                sole_range = Some((file, range));
            }
        }
        sole_range.expect("no range in fixture")
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
/// - points (`^`)
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
            if !line.trim_start().starts_with('#') {
                continue;
            }
            let mut range_start = None;
            for (column, char) in line.char_indices() {
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
                    points.push(Point { row: row - 1, column });
                }
            }

            if let Some(start) = range_start {
                ranges.push(start..Point { row: row - 1, column: line.len() });
            }
        }
        Self { points, ranges, text: fixture.to_owned() }
    }

    fn sole_point(&self) -> Option<Point> {
        if self.points.len() > 1 {
            panic!("more than one point in file fixture")
        }
        (self.points.len() == 1).then(|| self.points[0])
    }

    fn sole_range(&self) -> Option<std::ops::Range<Point>> {
        if self.ranges.len() > 1 {
            panic!("more than one range in file fixture")
        }
        (self.ranges.len() == 1).then(|| self.ranges[0].clone())
    }
}

#[cfg(test)]
mod tests;
