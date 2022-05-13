use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::ItemKind;
use gqls_syntax::{query, Query, QueryCursor};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::str::FromStr;
use vfs::FileId;

use crate::{Range, Snapshot};

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

#[macro_export]
macro_rules! error_msg {
    (E0001) => {
        "Syntax Error"
    };
    (E0002) => {
        "Unresolved directive `{}`"
    };
    ($ident:ident) => {
        compile_error!("unknown error code")
    };
}

#[macro_export]
macro_rules! diagnostic {
    ($code:ident @ $range:expr $(, $($arg:tt)* )? ) => {{
        let message = format!($crate::error_msg!($code) $( , $($arg)* )? );
        $crate::Diagnostic::new($range.into(), stringify!($code).parse().unwrap(), message)
    }};
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
        let items = self.snapshot.items(self.file);
        for (_, item) in items.iter() {
            let typedef = match item.kind {
                ItemKind::TypeDefinition(idx) => &items[idx],
                ItemKind::DirectiveDefinition(_) => continue,
            };
            for directive in typedef.directives.iter() {
                if self.snapshot.resolve_item(self.file, directive.name.clone()).is_empty() {
                    self.diagnostics
                        .insert(diagnostic!(E0002 @ directive.name.range, directive.name));
                }
            }
        }
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
            .map(|node| diagnostic!(E0001 @ node.range()));
        self.diagnostics.extend(diags);
    }
}

pub type Diagnostics = HashSet<Diagnostic>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Diagnostic {
    pub range: Range,
    pub code: ErrorCode,
    pub message: String,
}

impl Diagnostic {
    pub fn new(range: Range, code: ErrorCode, message: String) -> Self {
        Self { range, code, message }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ErrorCode(u16);

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl FromStr for ErrorCode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.starts_with("E"));
        Ok(Self(u16::from_str_radix(&s[1..], 10).expect("failed to parse error code")))
    }
}

#[cfg(test)]
mod tests;
