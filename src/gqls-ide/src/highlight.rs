use gqls_db::SourceDatabase;
use gqls_syntax::{NodeExt, NodeKind};
use vfs::FileId;

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
        let mut tokens = vec![];
        for node in gqls_syntax::traverse(&tree) {
            let kind = match node.kind() {
                // NodeKind::TYPE if node.is_named() => todo!("resolve type"),
                NodeKind::TYPE => SemanticTokenKind::Keyword,
                _ => continue,
            };
            tokens.push(SemanticToken { range: node.range().into(), kind });
        }
        tokens
    }
}

#[cfg(test)]
mod tests;
