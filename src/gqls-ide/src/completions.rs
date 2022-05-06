use std::fmt::{self, Debug};

use gqls_db::DefDatabase;
use gqls_ir::{ItemKind, TypeDefinitionKind};
use tree_sitter::Point;
use vfs::FileId;

use crate::Snapshot;

pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
}

impl Debug for CompletionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :: {:?}", self.label, self.kind)
    }
}

#[derive(Debug)]
pub enum CompletionItemKind {
    Object,
    Interface,
    Enum,
    Scalar,
    Union,
    Directive,
}

impl Snapshot {
    pub fn completions(&self, file: FileId, at: Point) -> Vec<CompletionItem> {
        CompletionCtxt::new(self, file, at).completions()
    }
}

struct CompletionCtxt<'s> {
    snapshot: &'s Snapshot,
    file: FileId,
    at: Point,
    completions: Vec<CompletionItem>,
}

impl<'s> CompletionCtxt<'s> {
    fn new(snapshot: &'s Snapshot, file: FileId, at: Point) -> Self {
        Self { snapshot, file, at, completions: Default::default() }
    }

    pub fn completions(mut self) -> Vec<CompletionItem> {
        self.complete_fields();
        self.completions
    }

    fn complete_fields(&mut self) {
        // FIXME just random implementation for now
        let project_items = self.snapshot.project_items(self.file);
        for items in project_items.values() {
            for (_, item) in items.items.iter() {
                let kind = match item.kind {
                    ItemKind::TypeDefinition(idx) => match items.typedefs[idx].kind {
                        TypeDefinitionKind::Object | TypeDefinitionKind::Input =>
                            CompletionItemKind::Object,
                        TypeDefinitionKind::Interface => CompletionItemKind::Interface,
                        TypeDefinitionKind::Scalar => CompletionItemKind::Scalar,
                        TypeDefinitionKind::Enum => CompletionItemKind::Enum,
                        TypeDefinitionKind::Union => CompletionItemKind::Union,
                    },
                    ItemKind::DirectiveDefinition(_) => CompletionItemKind::Directive,
                };
                self.completions.push(CompletionItem { label: item.name.to_string(), kind });
            }
        }
    }
}

#[cfg(test)]
mod tests;
