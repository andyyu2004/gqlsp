#![deny(rust_2018_idioms)]

mod def;
mod edit;
mod macros;

pub use self::edit::{Change, Changeset, Patch, Point, Range};
pub use tree_sitter;
pub use vfs::{Vfs, VfsPath};

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

use gqls_db::{FileData, GqlsDatabase, ParallelDatabase, SourceDatabase};
use gqls_parse::query;
use once_cell::sync::Lazy;
use ropey::Rope;
use std::fmt::{self, Display};
use tree_sitter::{Query, QueryCursor, TextProvider};

#[derive(Default)]
pub struct Ide {
    db: GqlsDatabase,
    vfs: Vfs,
    file_ropes: HashMap<VfsPath, Rope>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChangeSummary {
    pub path: VfsPath,
    pub diagnostics: HashSet<Diagnostic>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Diagnostic {
    pub range: Range,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(range: Range, kind: DiagnosticKind) -> Self {
        Self { range, kind }
    }

    pub fn syntax(range: Range) -> Self {
        Self::new(range, DiagnosticKind::Syntax)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DiagnosticKind {
    Syntax,
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax => write!(f, "Syntax Error"),
        }
    }
}

impl ChangeSummary {
    pub fn empty(path: VfsPath) -> Self {
        Self { path, diagnostics: Default::default() }
    }
}

pub struct Analysis {
    snapshot: gqls_db::Snapshot<GqlsDatabase>,
}

impl Analysis {
    pub fn syntax_tree(&self, path: VfsPath) -> String {
        self.snapshot.file_tree(path).root_node().to_sexp()
    }
}

impl Ide {
    pub fn analysis(&self) -> Analysis {
        Analysis { snapshot: self.db.snapshot() }
    }

    pub fn vfs(&self) -> &Vfs {
        &self.vfs
    }

    #[must_use]
    pub fn changeset<'a>(&mut self, path: &Path, changes: Vec<Change>) -> ChangeSummary {
        let changeset = self.make_changeset(path, changes);
        self.apply_changeset(changeset)
    }

    #[must_use]
    pub fn make_changeset<'a>(&mut self, path: &Path, changes: Vec<Change>) -> Changeset {
        Changeset::new(self.vfs.intern(path), changes)
    }

    #[must_use]
    pub fn apply_changesets<'a>(
        &mut self,
        changesets: impl IntoIterator<Item = Changeset>,
    ) -> Vec<ChangeSummary> {
        changesets.into_iter().map(|changeset| self.apply_changeset(changeset)).collect()
    }

    #[must_use]
    pub fn apply_changeset<'a>(&mut self, changeset: Changeset) -> ChangeSummary {
        let path = changeset.path;
        for change in changeset.changes {
            self.apply(path, &change);
        }
        let diagnostics = self.diagnostics(path);
        ChangeSummary { path, diagnostics }
    }

    fn apply(&mut self, file: VfsPath, change: &Change) {
        self.db.request_cancellation();
        self.patch_tree(file, change);
    }

    fn diagnostics(&self, file: VfsPath) -> HashSet<Diagnostic> {
        // can't query for missing nodes atm, so just traversing the entire tree to find any missing nodes
        static QUERY: Lazy<Query> = Lazy::new(|| query("(ERROR) @error"));
        let text = RopeText::new(&self.file_ropes[&file]);
        let mut cursor = QueryCursor::new();
        let tree = self.db.file_tree(file);
        cursor.set_match_limit(30);
        cursor
            .captures(&QUERY, tree.root_node(), text)
            .flat_map(|(captures, _)| captures.captures)
            .map(|capture| capture.node)
            .chain(gqls_parse::traverse(&tree).filter(|node| node.is_missing()))
            .map(|node| Diagnostic::syntax(node.range().into()))
            .collect()
    }

    fn patch_tree(&mut self, file: VfsPath, change: &Change) {
        let data = match &change {
            Change::Patch(patch) => {
                let mut rope = self.file_ropes.get(&file).cloned().expect("patch on initial edit");
                let edit = patch.apply(&mut rope);
                let text = rope.to_string();
                let mut old =
                    self.file_ropes.insert(file, rope).map(|_| self.db.file_tree(file)).unwrap();
                old.edit(&edit);
                let tree = gqls_parse::parse(&text, Some(&old));
                FileData::new(text, tree)
            }
            Change::Set(text) => {
                let rope = Rope::from_str(text);
                self.file_ropes.insert(file, rope);
                FileData::new(text, gqls_parse::parse_fresh(text))
            }
        };
        self.db.set_file_data(file, data);
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
