use gqls_db::DefDatabase;
use gqls_ir::{ItemKind, Name};
use vfs::FileId;

use crate::{Analysis, Range};

pub type SymbolTree = Vec<Symbol>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Symbol {
    pub name: Name,
    pub kind: SymbolKind,
    pub range: Range,
    pub children: SymbolTree,
    pub detail: Option<String>,
}

impl Symbol {
    pub fn new(name: Name, kind: SymbolKind, range: Range, children: SymbolTree) -> Self {
        Self { name, kind, range, children, detail: None }
    }

    pub fn leaf(name: Name, kind: SymbolKind, range: Range) -> Self {
        Self::new(name, kind, range, vec![])
    }

    pub fn with_detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SymbolKind {
    Struct,
    Field,
    Constant,
}

impl Analysis {
    pub fn document_symbols(&self, file: FileId) -> SymbolTree {
        let mut tree = SymbolTree::default();
        let items = self.items(file);
        for (idx, item) in items.items.iter() {
            let kind = match item.kind {
                // FIXME debatable which symbol kinds they should be
                ItemKind::TypeDefinition(_) | ItemKind::TypeExtension(_) => SymbolKind::Struct,
                ItemKind::DirectiveDefinition(_) => SymbolKind::Constant,
            };
            let children = self
                .item_body(file, idx)
                .as_ref()
                .and_then(|b| b.fields())
                .map(|fields| {
                    fields
                        .iter()
                        .map(|(_, field)| {
                            Symbol::leaf(field.name.clone(), SymbolKind::Field, field.range.into())
                                .with_detail(format!("{:?}", field.ty))
                        })
                        .collect()
                })
                .unwrap_or_default();

            let symbol = Symbol::new(items.name(item), kind, item.range.into(), children);
            tree.push(symbol);
        }
        tree
    }
}

#[cfg(test)]
mod tests;
