use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::{ItemKind, ItemRes, Name};
use vfs::FileId;

use crate::{Location, Range, Snapshot};

pub type SymbolTree = Vec<DocumentSymbol>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WorkspaceSymbol {
    pub name: Name,
    pub kind: SymbolKind,
    pub location: Location,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DocumentSymbol {
    pub name: Name,
    pub kind: SymbolKind,
    pub range: Range,
    pub children: SymbolTree,
    pub detail: Option<String>,
}

impl DocumentSymbol {
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

impl From<ItemKind> for SymbolKind {
    fn from(kind: ItemKind) -> Self {
        match kind {
            // FIXME debatable which symbol kinds they should be
            ItemKind::TypeDefinition(_) => SymbolKind::Struct,
            ItemKind::DirectiveDefinition(_) => SymbolKind::Constant,
        }
    }
}

impl Snapshot {
    // Symbols are not scoped to a project but across all of them
    pub fn workspace_symbols(&self) -> Vec<WorkspaceSymbol> {
        let mut symbols = vec![];
        for (file, items) in
            self.projects().iter().flat_map(|(_, fs)| fs).map(|file| (file, self.items(file)))
        {
            for (_, item) in items.items.iter() {
                symbols.push(WorkspaceSymbol {
                    name: item.name.clone(),
                    kind: item.kind.into(),
                    location: Location::new(file, item.range.into()),
                });
            }
        }
        symbols
    }

    pub fn document_symbols(&self, file: FileId) -> SymbolTree {
        let mut tree = SymbolTree::default();
        let items = self.items(file);
        for (idx, item) in items.items.iter() {
            let children = self
                .item_body(ItemRes { file, idx })
                .as_ref()
                .and_then(|b| b.fields())
                .map(|fields| {
                    fields
                        .iter()
                        .map(|(_, field)| {
                            DocumentSymbol::leaf(
                                field.name.clone(),
                                SymbolKind::Field,
                                field.range.into(),
                            )
                            .with_detail(format!("{:?}", field.ty))
                        })
                        .collect()
                })
                .unwrap_or_default();

            let symbol = DocumentSymbol::new(
                item.name.clone(),
                item.kind.into(),
                item.range.into(),
                children,
            );
            tree.push(symbol);
        }
        tree
    }
}

#[cfg(test)]
mod tests;
