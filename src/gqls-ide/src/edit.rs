use ropey::Rope;
pub use tree_sitter::Point;
use vfs::FileId;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Range {
    pub start: Point,
    pub end: Point,
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
        let new_end_position = Point {
            row: new_end_line,
            column: new_end_byte - rope.line_to_byte(new_end_line),
        };
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Change {
    pub file: FileId,
    pub kind: ChangeKind,
}

impl Change {
    pub fn new(file: FileId, kind: ChangeKind) -> Self {
        Self { file, kind }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ChangeKind {
    Patch(Patch),
    Set(String),
}

#[cfg(test)]
mod tests;
