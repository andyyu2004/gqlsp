use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_graphql() -> Language;
}

#[test]
fn test() {
    let language = unsafe { tree_sitter_graphql() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Range {
    /// The range's start position.
    pub start: Position,
    /// The range's end position.
    pub end: Position,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TextDocumentContentChangeEvent {
    /// The range of the document that changed.
    pub range: Option<Range>,

    /// The length of the range that got replaced.
    ///
    /// Deprecated: Use range instead
    pub range_length: Option<u32>,

    /// The new text of the document.
    pub text: String,
}

fn apply_edit(change: TextDocumentContentChangeEvent) {
    let range = change.range.unwrap();
    // NOTE
    // let old_len = range.end - range.start;
    tree_sitter::InputEdit {
        start_byte: todo!(),
        old_end_byte: todo!(),
        new_end_byte: todo!(),
        start_position: tree_sitter::Point {
            row: range.start.line as usize,
            column: range.start.character as usize,
        },
        old_end_position: tree_sitter::Point {
            row: range.end.line as usize,
            column: range.end.character as usize,
        },
        new_end_position: todo!(),
    };
}
