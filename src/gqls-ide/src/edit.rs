use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use gqls_db::Project;
pub use tree_sitter::Point;

use ropey::Rope;
use vfs::FileId;

/// Similar to [`tree_sitter::Range`] but only containing points (but no byte offsets)
#[derive(Eq, PartialEq, Copy, Clone, Hash, PartialOrd, Ord, Default)]
pub struct Range {
    pub start: Point,
    pub end: Point,
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}..{}:{}", self.start.row, self.start.column, self.end.row, self.end.column)
    }
}

pub trait RangeExt {
    fn contains(&self, point: Point) -> bool;
    fn intersects(&self, other: Self) -> bool;
}

impl RangeExt for Range {
    fn contains(&self, point: Point) -> bool {
        self.start <= point && point < self.end
    }

    fn intersects(&self, other: Self) -> bool {
        self.end.min(other.end) > self.start.max(other.start)
    }
}

impl From<std::ops::Range<Point>> for Range {
    fn from(range: std::ops::Range<Point>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

impl From<tree_sitter::Range> for Range {
    fn from(range: tree_sitter::Range) -> Self {
        Self { start: range.start_point, end: range.end_point }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FilePatches {
    pub file: FileId,
    pub patches: Vec<Patch>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Patch {
    pub range: Range,
    pub with: String,
}

impl Patch {
    #[must_use]
    pub fn apply(&self, rope: &mut Rope) -> tree_sitter::InputEdit {
        let Patch { range, with } = self;
        let start_byte = rope.line_to_byte(range.start.row) + range.start.column;
        let old_end_byte = rope.line_to_byte(range.end.row) + range.end.column;
        rope.remove(start_byte..old_end_byte);
        rope.insert(start_byte, with);
        let new_end_byte = rope.char_to_byte(start_byte + with.len());
        let new_end_line = rope.byte_to_line(new_end_byte);
        let new_end_position =
            Point { row: new_end_line, column: new_end_byte - rope.line_to_byte(new_end_line) };
        tree_sitter::InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position: range.start,
            old_end_position: range.end,
            new_end_position,
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct Changeset {
    pub(crate) projects: Option<HashMap<Project, HashSet<FileId>>>,
    pub(crate) changes: Vec<Change>,
}

impl Changeset {
    pub fn new(changes: Vec<Change>) -> Self {
        Self { changes, projects: None }
    }

    pub fn single(change: Change) -> Self {
        Self::new(vec![change])
    }

    pub fn with_projects(mut self, projects: HashMap<Project, HashSet<FileId>>) -> Self {
        self.projects = Some(projects);
        self
    }

    pub fn with_change(mut self, change: Change) -> Self {
        self.changes.push(change);
        self
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Change {
    pub file: FileId,
    pub kind: ChangeKind,
}

impl Change {
    pub fn new(file: FileId, kind: ChangeKind) -> Self {
        Self { file, kind }
    }

    pub fn set(file: FileId, text: String) -> Self {
        Self::new(file, ChangeKind::Set(text))
    }

    pub fn patch(file: FileId, patch: Patch) -> Self {
        Self::new(file, ChangeKind::Patch(patch))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ChangeKind {
    Patch(Patch),
    Set(String),
}

#[cfg(test)]
mod tests;
