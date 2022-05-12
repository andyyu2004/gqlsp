use std::collections::HashSet;
use std::fmt::{self, Debug};

use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::{DirectiveLocations, ItemKind, TypeDefinitionKind};
use gqls_syntax::{NodeExt, NodeKind, Position};
use vfs::FileId;

use crate::Snapshot;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
}

impl Debug for CompletionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :: {:?}", self.label, self.kind)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CompletionItemKind {
    Object,
    InputObject,
    Interface,
    Enum,
    Scalar,
    Union,
    Keyword,
    DirectiveLocation,
    Directive(DirectiveLocations),
}

impl Snapshot {
    pub fn completions(&self, position: Position) -> Vec<CompletionItem> {
        CompletionCtxt::new(self, position).completions()
    }
}

struct CompletionCtxt<'s> {
    snapshot: &'s Snapshot,
    file: FileId,
    context: Context,
    completions: Vec<CompletionItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    Document,
    InputField,
    Field,
    UnionMembers,
    DirectiveLocations,
    Interface,
    Directive(DirectiveLocations),
}

struct Queries {}

impl Default for Queries {
    fn default() -> Self {
        Self {}
    }
}

impl<'s> CompletionCtxt<'s> {
    fn infer_context(snapshot: &'s Snapshot, position: Position) -> Context {
        let data = snapshot.file_data(position.file);
        // NOTE maybe we could make use of treesitter's query api to do this better
        // HACK look backwards a few columns to try and find a notable node
        let mut point = position.point;
        for _ in 0..10 {
            let node = match data.tree.root_node().named_node_at(point) {
                Some(node) => node,
                None => return Context::Document,
            };
            match node.kind() {
                NodeKind::IMPLEMENTS_INTERFACES => return Context::Interface,
                NodeKind::OBJECT_TYPE_DEFINITION | NodeKind::OBJECT_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocations::OBJECT),
                NodeKind::ENUM_TYPE_DEFINITION | NodeKind::ENUM_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocations::ENUM),
                NodeKind::UNION_TYPE_DEFINITION | NodeKind::UNION_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocations::UNION),
                NodeKind::INTERFACE_TYPE_DEFINITION | NodeKind::INTERFACE_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocations::INTERFACE),
                NodeKind::SCALAR_TYPE_DEFINITION | NodeKind::SCALAR_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocations::SCALAR),
                NodeKind::INPUT_OBJECT_TYPE_DEFINITION | NodeKind::INPUT_OBJECT_TYPE_EXTENSION =>
                    return Context::Directive(DirectiveLocations::INPUT_OBJECT),
                NodeKind::DIRECTIVE_LOCATIONS | NodeKind::DIRECTIVE_LOCATION =>
                    return Context::DirectiveLocations,
                NodeKind::ENUM_VALUES_DEFINITION
                | NodeKind::ENUM_VALUE_DEFINITION
                | NodeKind::ENUM_VALUE =>
                    return Context::Directive(DirectiveLocations::ENUM_VALUE),
                NodeKind::INPUT_VALUE_DEFINITION => return Context::InputField,
                NodeKind::FIELD_DEFINITION => return Context::Field,
                NodeKind::UNION_MEMBER_TYPES => return Context::UnionMembers,
                NodeKind::NON_NULL_TYPE
                | NodeKind::LIST_TYPE
                | NodeKind::NAMED_TYPE
                | NodeKind::TYPE =>
                    if node.has_parent_of_kind(NodeKind::FIELD_DEFINITION) {
                        return Context::Field;
                    } else if node.has_parent_of_kind(NodeKind::INPUT_VALUE_DEFINITION) {
                        return Context::InputField;
                    } else if node.has_parent_of_kind(NodeKind::UNION_MEMBER_TYPES) {
                        return Context::UnionMembers;
                    } else if node.has_parent_of_kind(NodeKind::IMPLEMENTS_INTERFACES) {
                        return Context::Interface;
                    },
                _ => {
                    if point.column == 0 {
                        break;
                    }
                    point.column -= 1;
                }
            }
        }

        Context::Document
    }

    fn new(snapshot: &'s Snapshot, position: Position) -> Self {
        let context = Self::infer_context(snapshot, position);
        tracing::info!("inferred completion context: {:?}", context);
        Self { snapshot, file: position.file, context, completions: Default::default() }
    }

    pub fn completions(mut self) -> Vec<CompletionItem> {
        match self.context {
            Context::Field => self.complete_fields(),
            Context::Document => self.complete_document(),
            Context::UnionMembers => self.complete_union_member(),
            Context::InputField => self.complete_input_fields(),
            Context::Directive(location) => self.complete_directives(location),
            Context::DirectiveLocations => self.complete_directive_locations(),
            Context::Interface => self.complete_interfaces(),
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
        let project_items = self.snapshot.project_items(self.file);
        let mut completions = HashSet::new();
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
                    ItemKind::DirectiveDefinition(idx) =>
                        CompletionItemKind::Directive(items.directives[idx].locations),
                };

                let label = match item.kind {
                    ItemKind::TypeDefinition(_) => item.name.to_string(),
                    ItemKind::DirectiveDefinition(_) => format!("@{}", item.name),
                };
                completions.insert(CompletionItem { label, kind });
            }
        }
        let mut v = completions.into_iter().collect::<Vec<_>>();
        v.sort();
        v.into_iter()
    }

    fn complete_input_fields(&mut self) {
        self.completions.extend(self.items().filter(|item| match item.kind {
            CompletionItemKind::Directive(loc) =>
                loc.contains(DirectiveLocations::INPUT_FIELD_DEFINITION),
            CompletionItemKind::InputObject
            | CompletionItemKind::Enum
            | CompletionItemKind::Scalar => true,
            CompletionItemKind::Object
            | CompletionItemKind::DirectiveLocation
            | CompletionItemKind::Interface
            | CompletionItemKind::Union
            | CompletionItemKind::Keyword => false,
        }));
    }

    fn complete_fields(&mut self) {
        self.completions.extend(self.items().filter(|item| match item.kind {
            CompletionItemKind::Directive(loc) =>
                loc.contains(DirectiveLocations::FIELD_DEFINITION),
            CompletionItemKind::Object
            | CompletionItemKind::Interface
            | CompletionItemKind::Enum
            | CompletionItemKind::Scalar
            | CompletionItemKind::Union => true,
            CompletionItemKind::InputObject
            | CompletionItemKind::Keyword
            | CompletionItemKind::DirectiveLocation => false,
        }));
    }

    fn complete_union_member(&mut self) {
        self.completions
            .extend(self.items().filter(|item| matches!(item.kind, CompletionItemKind::Object)))
    }

    fn complete_directives(&mut self, location: DirectiveLocations) {
        let completions = self.items().filter(|item| matches!(item.kind, CompletionItemKind::Directive(locations) if locations.contains(location)));
        self.completions.extend(completions);
    }

    fn complete_directive_locations(&mut self) {
        // FIXME once bitflags allows iteration
        // DirectiveLocations::all().iter();
        self.completions.extend(
            [
                "SCHEMA",
                "SCALAR",
                "OBJECT",
                "FIELD_DEFINITION",
                "ARGUMENT_DEFINITION",
                "INTERFACE",
                "UNION",
                "ENUM",
                "ENUM_VALUE",
                "INPUT_OBJECT",
                "INPUT_FIELD_DEFINITION",
            ]
            .map(|s| CompletionItem {
                label: s.to_owned(),
                kind: CompletionItemKind::DirectiveLocation,
            }),
        )
    }

    fn complete_interfaces(&mut self) {
        self.completions
            .extend(self.items().filter(|item| matches!(item.kind, CompletionItemKind::Interface)));
    }
}

#[cfg(test)]
mod tests;
