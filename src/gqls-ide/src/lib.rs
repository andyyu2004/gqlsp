#![deny(rust_2018_idioms)]

mod completions;
mod def;
mod diagnostics;
mod edit;
mod highlight;
mod implementation;
mod macros;
mod references;
mod rename;
mod resolve;
mod symbols;
mod typedef;

pub use self::completions::{CompletionItem, CompletionItemKind};
pub use self::diagnostics::{Diagnostic, DiagnosticLabel, Diagnostics, ErrorCode, Severity};
pub use self::edit::{Change, ChangeKind, Changeset, FilePatches, Patch, Point, Range};
pub use self::highlight::{SemanticToken, SemanticTokenKind};
pub use self::rename::RenameError;
pub use self::symbols::{DocumentSymbol, SymbolKind, SymbolTree, WorkspaceSymbol};
pub use gqls_syntax::{Position, RangeExt};
use parking_lot::RwLock;
pub use tree_sitter;
pub use vfs::{FileId, Vfs};

use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gqls_db::{Database, FileData, GqlsDatabase, ParallelDatabase, Project, SourceDatabase};
use once_cell::sync::Lazy;
use ropey::Rope;
use std::fmt::{self, Debug};

// bit of a hack, there is probably a nicer way (we need access to the interner for `path` related conversion)
pub static VFS: Lazy<RwLock<Vfs>> = Lazy::new(Default::default);

#[derive(Default)]
pub struct Ide {
    db: GqlsDatabase,
    file_ropes: HashMap<FileId, Rope>,
}

pub type ChangesetSummary = HashMap<FileId, ChangeSummary>;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ChangeSummary {
    pub diagnostics: HashSet<Diagnostic>,
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

// Wrapper that hides the static variable
pub struct VfsProxy;

impl VfsProxy {
    pub fn intern(&self, path: impl AsRef<Path>) -> FileId {
        VFS.write().intern(path)
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<FileId> {
        VFS.read().get(path)
    }
}

impl Ide {
    pub fn snapshot(&self) -> Snapshot {
        self.db.unwind_if_cancelled();
        Snapshot { snapshot: self.db.snapshot() }
    }

    pub fn intern_path(&mut self, path: PathBuf) -> FileId {
        self.vfs().intern(path)
    }

    pub fn intern_project(&self, project: String) -> Project {
        self.db.intern_project(project)
    }

    pub fn vfs(&self) -> VfsProxy {
        VfsProxy
    }

    #[must_use]
    pub fn apply_changeset<'a>(&mut self, changeset: Changeset) -> ChangesetSummary {
        self.db.request_cancellation();
        if let Some(projects) = changeset.projects {
            self.db.set_projects(Arc::new(projects));
        }

        changeset.changes.iter().for_each(|change| {
            self.apply(change);
        });

        let files_changed =
            changeset.changes.iter().map(|change| change.file).collect::<HashSet<_>>();
        files_changed
            .into_iter()
            .map(|path| (path, ChangeSummary { diagnostics: self.snapshot().diagnostics(path) }))
            .collect()
    }

    fn apply(&mut self, change: &Change) {
        self.patch_tree(change);
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

#[derive(Eq, PartialEq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Location {
    pub file: FileId,
    pub range: Range,
}

impl Location {
    pub fn new(file: FileId, range: Range) -> Self {
        Self { file, range }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:?}", self.file.display(), self.range)
    }
}

#[cfg(test)]
mod tests;
