#![deny(rust_2018_idioms)]

mod edit;

pub use self::edit::{Change, Changeset, Patch, Point, Range};

use std::collections::HashMap;
use std::path::Path;

use gqls_db::{GqlsDatabase, SourceDatabase};
use gqls_parse::query;
use once_cell::sync::Lazy;
use ropey::Rope;
use tree_sitter::{Query, QueryCursor, TextProvider};
use vfs::{FileId, Vfs};

#[derive(Default)]
pub struct Ide {
    db: GqlsDatabase,
    vfs: Vfs,
    file_ropes: HashMap<FileId, Rope>,
}

#[derive(Debug)]
pub struct ChangeSummary {
    pub file: FileId,
    pub error_ranges: Vec<Range>,
}

impl ChangeSummary {
    pub fn new(file: FileId) -> Self {
        Self { file, error_ranges: vec![] }
    }
}

impl ChangeSummary {
    fn aggregate(&mut self, other: &Self) {
        let &Self { file, ref error_ranges } = other;
        assert_eq!(self.file, file);
        self.error_ranges.extend_from_slice(error_ranges);
    }
}

impl Ide {
    pub fn vfs(&self) -> &Vfs {
        &self.vfs
    }

    pub fn make_changeset<'a>(&mut self, path: &Path, changes: Vec<Change>) -> Changeset {
        Changeset::new(self.vfs.intern(path), changes)
    }

    pub fn apply_changesets<'a>(
        &mut self,
        changesets: impl IntoIterator<Item = Changeset>,
    ) -> Vec<ChangeSummary> {
        changesets.into_iter().map(|changeset| self.apply_changeset(changeset)).collect()
    }

    #[must_use]
    pub fn apply_changeset<'a>(&mut self, changeset: Changeset) -> ChangeSummary {
        let mut summary = ChangeSummary::new(changeset.file);
        for change in changeset.changes {
            summary.aggregate(&self.apply(changeset.file, &change));
        }
        summary
    }

    pub fn apply(&mut self, file: FileId, change: &Change) -> ChangeSummary {
        self.db.request_cancellation();
        self.patch_tree(file, change);
        let error_ranges = self.error_ranges(file);
        ChangeSummary { file, error_ranges }
    }

    fn error_ranges(&self, file: FileId) -> Vec<Range> {
        static QUERY: Lazy<Query> = Lazy::new(|| query("(ERROR)"));
        let text = RopeText::new(&self.file_ropes[&file]);
        let mut cursor = QueryCursor::new();
        let tree = self.db.file_tree(file);
        cursor.set_match_limit(30);
        cursor
            .matches(&QUERY, tree.root_node(), text)
            .flat_map(|query_match| query_match.captures)
            .map(|capture| capture.node.range().into())
            .collect()
    }

    fn patch_tree(&mut self, file: FileId, change: &Change) {
        let tree = match &change {
            Change::Patch(patch) => {
                let mut rope = self.file_ropes.get(&file).cloned().expect("patch on initial edit");
                let edit = patch.apply(&mut rope);
                let text = rope.to_string();
                let mut old =
                    self.file_ropes.insert(file, rope).map(|_| self.db.file_tree(file)).unwrap();
                old.edit(&edit);
                gqls_parse::parse(&text, Some(&old))
            }
            Change::Set(text) => {
                let rope = Rope::from_str(text);
                self.file_ropes.insert(file, rope);
                gqls_parse::parse(text, None)
            }
        };
        self.db.set_file_tree(file, tree.clone());
    }
}

struct RopeText<'a>(&'a Rope);

impl<'a> RopeText<'a> {
    pub fn new(rope: &'a Rope) -> Self {
        RopeText(rope)
    }
}

impl<'a> TextProvider<'a> for RopeText<'a> {
    type I = std::iter::Map<ropey::iter::Chunks<'a>, fn(&str) -> &[u8]>;

    fn text(&mut self, node: tree_sitter::Node<'_>) -> Self::I {
        self.0.byte_slice(node.byte_range()).chunks().map(str::as_bytes)
    }
}

#[cfg(test)]
mod tests;
