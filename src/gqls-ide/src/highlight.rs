use gqls_db::SourceDatabase;
use gqls_syntax::{Node, NodeExt, NodeKind};
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
    Interface,
    Struct,
    Enum,
    Const,
    Keyword,
    Type,
}

impl Snapshot {
    pub fn semantic_tokens(&self, file: FileId) -> Vec<SemanticToken> {
        let tree = self.file_tree(file);
        return Highlighter::new(self, gqls_syntax::traverse_preorder(&tree)).highlight();
    }

    fn highlight_type(node: Node<'_>) -> SemanticTokenKind {
        SemanticTokenKind::Interface
    }
}

struct Highlighter<'a, 'tree> {
    snapshot: &'a Snapshot,
    nodes: Box<dyn Iterator<Item = Node<'tree>> + 'tree>,
    tokens: Vec<SemanticToken>,
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
    fn new(snapshot: &'a Snapshot, nodes: impl Iterator<Item = Node<'tree>> + 'tree) -> Self {
        Self { snapshot, nodes: Box::new(nodes), tokens: Default::default() }
    }

    fn highlight(mut self) -> Vec<SemanticToken> {
        self.highlight_document();
        self.tokens
    }

    fn highlight_document(&mut self) {
        let node = next!(self);
        let kind = match node.kind() {
            NodeKind::TYPE if !node.is_named() => SemanticTokenKind::Keyword,
            NodeKind::TYPE => todo!(),
            // TODO resolve type
            NodeKind::NAME => SemanticTokenKind::Type,
            _ => todo!(),
        };

        let range = Range::from(node.range());
        #[cfg(debug_assertions)]
        if let Some(prev) = self.tokens.last() {
            assert!(!prev.range.intersects(range));
        }
        self.tokens.push(SemanticToken { range, kind });
    }
}

#[cfg(test)]
mod tests;
