use std::collections::HashSet;
use std::fmt::{self, Display};

use gqls_db::SourceDatabase;
use gqls_syntax::{query, Query, QueryCursor};
use once_cell::sync::Lazy;
use vfs::FileId;

use crate::{Range, Snapshot};

pub type Diagnostics = HashSet<Diagnostic>;

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

impl Snapshot {
    pub fn diagnostics(&self, file: FileId) -> Diagnostics {
        DiagnosticsCtxt::new(self, file).diagnostics()
    }
}

struct DiagnosticsCtxt<'a> {
    snapshot: &'a Snapshot,
    file: FileId,
    diagnostics: HashSet<Diagnostic>,
}

impl<'a> DiagnosticsCtxt<'a> {
    fn new(snapshot: &'a Snapshot, file: FileId) -> Self {
        Self { snapshot, file, diagnostics: Default::default() }
    }

    fn diagnostics(mut self) -> HashSet<Diagnostic> {
        self.syntax();
        self.unresolved_references();
        self.diagnostics
    }

    fn unresolved_references(&mut self) {
    }

    fn syntax(&mut self) {
        // can't query for missing nodes atm, so just traversing the entire tree to find any missing nodes
        static QUERY: Lazy<Query> = Lazy::new(|| query("(ERROR) @error"));
        let mut cursor = QueryCursor::new();
        let data = self.snapshot.file_data(self.file);
        let tree = data.tree;
        cursor.set_match_limit(30);
        let diags = cursor
            .captures(&QUERY, tree.root_node(), data.text.as_bytes())
            .flat_map(|(captures, _)| captures.captures)
            .map(|capture| capture.node)
            .chain(gqls_syntax::traverse_preorder(&tree).filter(|node| node.is_missing()))
            .map(|node| Diagnostic::syntax(node.range().into()));
        self.diagnostics.extend(diags);
    }
}

#[cfg(test)]
mod tests;
