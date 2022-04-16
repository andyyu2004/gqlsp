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
                $(vfs.intern($file) => $crate::FixtureFile::parse($text)),*
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

    pub fn fileset(&self) -> HashSet<FileId> {
        self.files.keys().copied().collect()
    }

    pub fn files(&self) -> &HashMap<FileId, FixtureFile> {
        &self.files
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A fixture file supports two forms of annotations: points (`^`) and ranges (`...`)
pub struct FixtureFile {
    pub points: Vec<Point>,
    pub ranges: Vec<std::ops::Range<Point>>,
    pub text: String,
}

impl FixtureFile {
    pub fn parse(fixture: &str) -> Self {
        let mut points = vec![];
        let mut ranges = vec![];
        for (row, line) in fixture.lines().enumerate() {
            let mut range_start = None;
            for (column, char) in line.char_indices() {
                let point = Point { row: row - 1, column };
                if char == '.' {
                    if row == 0 {
                        panic!("cannot contain `.` in the first line");
                    }
                    if range_start.is_none() {
                        range_start = Some(point);
                    }
                } else if let Some(start) = range_start.take() {
                    ranges.push(start..point)
                }

                if char == '^' {
                    if row == 0 {
                        panic!("cannot contain `^` in the first line");
                    }
                    points.push(point);
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
}

#[cfg(test)]
mod tests;
