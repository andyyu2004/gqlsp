use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::ItemKind;
use gqls_syntax::{Node, NodeExt, NodeKind, Traverse, TraverseEvent};
use vfs::FileId;

use crate::edit::RangeExt;
use crate::{Range, Snapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticToken {
    range: Range,
    kind: SemanticTokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticTokenKind {
    Comment,
    Directive,
    Enum,
    Interface,
    Keyword,
    Number,
    Object,
    Scalar,
    String,
    Type,
}

impl Snapshot {
    pub fn semantic_tokens(&self, file: FileId) -> Vec<SemanticToken> {
        let tree = self.file_tree(file);
        return Highlighter::new(self, file, gqls_syntax::traverse(&tree)).highlight();
    }

    fn highlight_type(node: Node<'_>) -> SemanticTokenKind {
        SemanticTokenKind::Interface
    }
}

struct Highlighter<'a, 'tree> {
    snapshot: &'a Snapshot,
    file: FileId,
    nodes: Traverse<'tree>,
    tokens: Vec<SemanticToken>,
    scopes: Vec<&'static str>,
}

macro_rules! next {
    ($self:ident) => {{
        match $self.nodes.next() {
            Some(node) => node,
            None => return,
        }
    }};
}

impl<'a, 'tree> Highlighter<'a, 'tree> {
    fn new(snapshot: &'a Snapshot, file: FileId, nodes: Traverse<'tree>) -> Self {
        Self { snapshot, file, nodes, tokens: Default::default(), scopes: Default::default() }
    }

    fn highlight(mut self) -> Vec<SemanticToken> {
        self.highlight_document();
        self.tokens
    }

    fn highlight_document(&mut self) {
        let mut skip_until = None;
        loop {
            // while let Some() if we can keep this structure
            let event = next!(self);
            let node = event.node();

            let scope = match node.kind() {
                NodeKind::FIELD_DEFINITION => Some(node.kind()),
                _ => None,
            };

            if let Some(scope) = scope {
                match event {
                    TraverseEvent::Enter(_) => self.scopes.push(scope),
                    TraverseEvent::Exit(_) => {
                        assert_eq!(self.scopes.pop().unwrap(), scope);
                    }
                }
            }

            if skip_until.is_some() || event.is_exit() {
                if Some(event.node().kind()) == skip_until {
                    skip_until = None;
                }
                continue;
            }

            let at = node.range().start_point;
            let kind = match node.kind() {
                NodeKind::TYPE if !node.is_named() => SemanticTokenKind::Keyword,
                // TODO builtin types (ID, String, Int should be defaultLibrary types)
                NodeKind::TYPE => match self.snapshot.resolve_type_at(self.file, at)[..] {
                    [] => SemanticTokenKind::Type,
                    [x, ..] => match self.snapshot.item(x).kind {
                        ItemKind::TypeDefinition(_) => SemanticTokenKind::Object,
                        ItemKind::DirectiveDefinition(_) => SemanticTokenKind::Directive,
                    },
                },
                // FIXME
                NodeKind::NAME => SemanticTokenKind::Type,
                _ => continue,
            };

            let range = Range::from(node.range());
            let token = SemanticToken { range, kind };
            #[cfg(debug_assertions)]
            if let Some(prev) = self.tokens.last() {
                assert!(prev.range.end <= range.start, "token range overlap {prev:?}, {token:?}",);
                assert!(!prev.range.intersects(range));
            }
            skip_until = Some(node.kind());
            self.tokens.push(token);
        }
    }
}

#[cfg(test)]
mod tests;
