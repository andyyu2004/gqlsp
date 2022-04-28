#![deny(rust_2018_idioms)]

mod def;
mod edit;
mod highlight;
mod implementation;
mod macros;
mod references;
mod resolve;
mod symbols;
mod typedef;

pub use self::edit::{Change, ChangeKind, Changeset, Patch, Point, Range};
pub use self::highlight::SemanticToken;
pub use self::symbols::{Symbol, SymbolKind, SymbolTree};
pub use tree_sitter;
pub use vfs::{FileId, Vfs};

use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use gqls_db::{FileData, GqlsDatabase, ParallelDatabase, Project, SourceDatabase};
use gqls_syntax::query;
use once_cell::sync::Lazy;
use ropey::Rope;
use std::fmt::{self, Display};
use tree_sitter::{Query, QueryCursor, TextProvider};

#[derive(Default)]
pub struct Ide {
    db: GqlsDatabase,
    vfs: Vfs,
    file_ropes: HashMap<FileId, Rope>,
}

pub type ChangesetSummary = HashMap<FileId, ChangeSummary>;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ChangeSummary {
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

pub struct Snapshot {
    snapshot: gqls_db::Snapshot<GqlsDatabase>,
}

impl Deref for Snapshot {
    type Target = gqls_db::Snapshot<GqlsDatabase>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl Snapshot {
    pub fn syntax_tree(&self, file: FileId) -> String {
        self.file_tree(file).root_node().to_sexp()
    }
}

impl Ide {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot { snapshot: self.db.snapshot() }
    }

    pub fn intern_path(&mut self, path: PathBuf) -> FileId {
        self.vfs.intern(path)
    }

    pub fn intern_project(&self, project: String) -> Project {
        self.db.intern_project(project)
    }

    pub fn vfs(&self) -> &Vfs {
        &self.vfs
    }

    #[must_use]
    pub fn apply_changeset<'a>(&mut self, changeset: Changeset) -> ChangesetSummary {
        self.db.request_cancellation();
        if let Some(projects) = changeset.projects {
            self.db.set_projects(Arc::new(projects));
        }

        changeset.changes.iter().for_each(|change| {
            self.apply(&change);
        });

        let files_changed =
            changeset.changes.iter().map(|change| change.file).collect::<HashSet<_>>();
        files_changed
            .into_iter()
            .map(|path| (path, ChangeSummary { diagnostics: self.diagnostics(path) }))
            .collect()
    }

    fn apply(&mut self, change: &Change) {
        self.patch_tree(change);
    }

    fn diagnostics(&self, file: FileId) -> HashSet<Diagnostic> {
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
            .chain(gqls_syntax::traverse_preorder(&tree).filter(|node| node.is_missing()))
            .map(|node| Diagnostic::syntax(node.range().into()))
            .collect()
    }

    fn patch_tree(&mut self, change: &Change) {
        let file = change.file;
        let data = match &change.kind {
            ChangeKind::Patch(patch) => {
                let mut rope = self.file_ropes.get(&file).cloned().expect("patch on initial edit");
                let edit = patch.apply(&mut rope);
                let text = rope.to_string();
                let mut old =
                    self.file_ropes.insert(file, rope).map(|_| self.db.file_tree(file)).unwrap();
                old.edit(&edit);
                let tree = gqls_syntax::parse(&text, Some(&old));
                FileData::new(text, tree)
            }
            ChangeKind::Set(text) => {
                let rope = Rope::from_str(text);
                self.file_ropes.insert(file, rope);
                FileData::new(text, gqls_syntax::parse_fresh(text))
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Location {
    pub file: FileId,
    pub range: Range,
}

impl Location {
    pub fn new(file: FileId, range: Range) -> Self {
        Self { file, range }
    }
}

#[cfg(test)]
mod tests;
