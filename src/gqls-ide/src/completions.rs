use std::fmt::{self, Debug};

use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::{DirectiveLocation, ItemKind, TypeDefinitionKind};
use gqls_syntax::{NodeExt, NodeKind};
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

#[derive(Debug, PartialEq, Eq)]
pub enum CompletionItemKind {
    Object,
    InputObject,
    Interface,
    Enum,
    Scalar,
    Union,
    Directive,
    Keyword,
}

impl Snapshot {
    pub fn completions(&self, file: FileId, at: Point) -> Vec<CompletionItem> {
        CompletionCtxt::new(self, file, at).completions()
    }
}

struct CompletionCtxt<'s> {
    snapshot: &'s Snapshot,
    file: FileId,
    context: Context,
    completions: Vec<CompletionItem>,
}

#[derive(Debug, PartialEq, Eq)]
enum Context {
    Document,
    InputField,
    Field,
    UnionMembers,
    Directive(DirectiveLocation),
}

struct Queries {}

impl Default for Queries {
    fn default() -> Self {
        Self {}
    }
}
impl<'s> CompletionCtxt<'s> {
    fn infer_context(snapshot: &'s Snapshot, file: FileId, mut at: Point) -> Context {
        let data = snapshot.file_data(file);
        // NOTE maybe we could make use of treesitter's query api to do this better
        // HACK look backwards a few columns to try and find a notable node
        for _ in 0..10 {
            let node = match data.tree.root_node().named_node_at(at) {
                Some(node) => node,
                None => return Context::Document,
            };
            dbg!(node.to_sexp());
            match node.kind() {
                NodeKind::OBJECT_TYPE_DEFINITION | NodeKind::OBJECT_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocation::Object),
                NodeKind::ENUM_TYPE_DEFINITION | NodeKind::ENUM_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocation::Enum),
                NodeKind::UNION_TYPE_DEFINITION | NodeKind::UNION_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocation::Union),
                NodeKind::INTERFACE_TYPE_DEFINITION | NodeKind::INTERFACE_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocation::Interface),
                NodeKind::SCALAR_TYPE_DEFINITION | NodeKind::SCALAR_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocation::Scalar),
                NodeKind::INPUT_OBJECT_TYPE_DEFINITION | NodeKind::INPUT_OBJECT_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocation::InputObject),
                NodeKind::ENUM_VALUES_DEFINITION
                | NodeKind::ENUM_VALUE_DEFINITION
                | NodeKind::ENUM_VALUE => return Context::Directive(DirectiveLocation::EnumValue),
                NodeKind::INPUT_FIELDS_DEFINITION => return Context::InputField,
                NodeKind::FIELDS_DEFINITION | NodeKind::FIELD_DEFINITION => return Context::Field,
                NodeKind::UNION_MEMBER_TYPES => return Context::UnionMembers,
                _ => {
                    if at.column == 0 {
                        break;
                    }
                    at.column -= 1;
                }
            }
        }

        return Context::Document;
    }

    fn new(snapshot: &'s Snapshot, file: FileId, at: Point) -> Self {
        let context = Self::infer_context(snapshot, file, at);
        Self { snapshot, file, context, completions: Default::default() }
    }

    pub fn completions(mut self) -> Vec<CompletionItem> {
        match self.context {
            Context::Field => self.complete_fields(),
            Context::Document => self.complete_document(),
            Context::UnionMembers => self.complete_union_member(),
            Context::InputField => self.complete_input_fields(),
            Context::Directive(location) => self.complete_directives(location),
        }
        self.completions
    }

    fn complete_document(&mut self) {
        self.completions
            .extend(["scalar", "enum", "struct", "union", "interface", "directive", "input"].map(
                |s| CompletionItem { label: s.to_owned(), kind: CompletionItemKind::Keyword },
            ));
    }

    fn items(&self) -> impl Iterator<Item = CompletionItem> {
        // FIXME use a proper iterative approach
        let project_items = self.snapshot.project_items(self.file);
        let mut completions = vec![];
        for items in project_items.values() {
            for (_, item) in items.items.iter() {
                let kind = match item.kind {
                    ItemKind::TypeDefinition(idx) => match items.typedefs[idx].kind {
                        TypeDefinitionKind::Object => CompletionItemKind::Object,

                        TypeDefinitionKind::Input => CompletionItemKind::InputObject,
                        TypeDefinitionKind::Interface => CompletionItemKind::Interface,
                        TypeDefinitionKind::Scalar => CompletionItemKind::Scalar,
                        TypeDefinitionKind::Enum => CompletionItemKind::Enum,
                        TypeDefinitionKind::Union => CompletionItemKind::Union,
                    },
                    ItemKind::DirectiveDefinition(_) => CompletionItemKind::Directive,
                };
                completions.push(CompletionItem { label: item.name.to_string(), kind });
            }
        }
        completions.into_iter()
    }

    fn complete_input_fields(&mut self) {
        self.completions.extend(self.items().filter(|item| match item.kind {
            CompletionItemKind::InputObject
            | CompletionItemKind::Enum
            | CompletionItemKind::Scalar => true,
            CompletionItemKind::Object
            | CompletionItemKind::Interface
            | CompletionItemKind::Union
            | CompletionItemKind::Directive
            | CompletionItemKind::Keyword => false,
        }));
    }

    fn complete_fields(&mut self) {
        self.completions.extend(self.items().filter(|item| match item.kind {
            CompletionItemKind::Object
            | CompletionItemKind::Interface
            | CompletionItemKind::Enum
            | CompletionItemKind::Scalar
            | CompletionItemKind::Union => true,
            CompletionItemKind::Directive
            | CompletionItemKind::InputObject
            | CompletionItemKind::Keyword => false,
        }));
    }

    fn complete_union_member(&mut self) {
        self.completions
            .extend(self.items().filter(|item| matches!(item.kind, CompletionItemKind::Object)))
    }

    fn complete_directives(&mut self, location: DirectiveLocation) {
        // TODO
    }
}

#[cfg(test)]
mod tests;
