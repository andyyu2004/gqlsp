use std::sync::Arc;

use gqls_parse::{Node, NodeExt, NodeKind, Tree};
use la_arena::ArenaMap;

use crate::*;

pub(crate) struct BodyCtxt {
    text: Arc<str>,
}

impl BodyCtxt {
    pub(crate) fn new(text: Arc<str>) -> Self {
        Self { text }
    }

    pub fn lower_typedef(mut self, node: Node<'_>) -> ItemBody {
        match node.kind() {
            NodeKind::OBJECT_TYPE_DEFINITION =>
                ItemBody::ObjectTypeDefinition(self.lower_object_typedef(node)),
            NodeKind::INTERFACE_TYPE_DEFINITION =>
                ItemBody::InterfaceDefinition(self.lower_interface_typedef(node)),
            NodeKind::INPUT_OBJECT_TYPE_DEFINITION =>
                ItemBody::InputObjectTypeDefinition(self.lower_input_object_typedef(node)),
            _ => ItemBody::Todo,
        }
    }

    pub(crate) fn lower_type_ext(mut self, node: Node<'_>) -> ItemBody {
        match node.kind() {
            NodeKind::OBJECT_TYPE_EXTENSION =>
                ItemBody::ObjectTypeExtension(self.lower_object_type_ext(node)),
            _ => ItemBody::Todo,
        }
    }

    fn lower_object_typedef(&mut self, node: Node<'_>) -> TypeDefinitionBody {
        assert_eq!(node.kind(), NodeKind::OBJECT_TYPE_DEFINITION);
        TypeDefinitionBody { fields: self.lower_fields_of(node) }
    }

    fn lower_object_type_ext(&mut self, node: Node<'_>) -> TypeExtensionBody {
        assert_eq!(node.kind(), NodeKind::OBJECT_TYPE_EXTENSION);
        TypeExtensionBody { fields: self.lower_fields_of(node) }
    }

    fn lower_input_object_typedef(&mut self, node: Node<'_>) -> InputTypeDefinitionBody {
        assert_eq!(node.kind(), NodeKind::INPUT_OBJECT_TYPE_DEFINITION);
        let fields = node
            .child_of_kind(NodeKind::INPUT_FIELDS_DEFINITION)
            .map(|fields| self.lower_input_fields(fields))
            .unwrap_or_default();
        InputTypeDefinitionBody { fields }
    }

    fn lower_interface_typedef(&mut self, node: Node<'_>) -> InterfaceDefinitionBody {
        assert_eq!(node.kind(), NodeKind::INTERFACE_TYPE_DEFINITION);
        InterfaceDefinitionBody { fields: self.lower_fields_of(node) }
    }

    fn lower_fields_of(&mut self, node: Node<'_>) -> Fields {
        node.child_of_kind(NodeKind::FIELDS_DEFINITION)
            .map(|fields| self.lower_fields(fields))
            .unwrap_or_default()
    }

    fn lower_input_fields(&mut self, node: Node<'_>) -> InputFields {
        assert_eq!(node.kind(), NodeKind::INPUT_FIELDS_DEFINITION);
        let cursor = &mut node.walk();
        let fields = node
            .children_of_kind(cursor, NodeKind::INPUT_VALUE_DEFINITION)
            .filter_map(|field| self.lower_input_field(field))
            .map(|(field, _default_value)| field);
        InputFields::new(
            // TODO
            fields,
            ArenaMap::default(),
        )
    }

    fn lower_input_field(&mut self, node: Node<'_>) -> Option<(Field, ())> {
        assert_eq!(node.kind(), NodeKind::INPUT_VALUE_DEFINITION);
        let name = self.name_of(node)?;
        let ty = self.lower_type(node.child_of_kind(NodeKind::TYPE)?)?;
        let directives = self.lower_directives_of(node);
        // TODO default_value
        Some((Field { range: node.range(), name, ty, directives }, ()))
    }

    fn lower_fields(&mut self, node: Node<'_>) -> Fields {
        assert_eq!(node.kind(), NodeKind::FIELDS_DEFINITION);
        Fields::new(
            node.children_of_kind(&mut node.walk(), NodeKind::FIELD_DEFINITION)
                .filter_map(|field| self.lower_field(field)),
        )
    }

    fn lower_field(&mut self, node: Node<'_>) -> Option<Field> {
        assert_eq!(node.kind(), NodeKind::FIELD_DEFINITION);
        let ty = self.lower_type(node.child_of_kind(NodeKind::TYPE)?)?;
        let name = self.name_of(node)?;
        let directives = self.lower_directives_of(node);
        Some(Field { range: node.range(), name, ty, directives })
    }

    fn lower_type(&mut self, node: Node<'_>) -> Option<Ty> {
        assert_eq!(node.kind(), NodeKind::TYPE);
        let ty = node.sole_named_child();
        let kind = match ty.kind() {
            NodeKind::NAMED_TYPE => return self.lower_named_type(ty),
            NodeKind::LIST_TYPE => TyKind::List(self.lower_type(ty.sole_named_child())?),
            NodeKind::NON_NULL_TYPE => {
                let inner = ty.sole_named_child();
                match inner.kind() {
                    NodeKind::NAMED_TYPE => TyKind::NonNull(self.lower_named_type(inner)?),
                    NodeKind::LIST_TYPE => TyKind::NonNull(self.lower_list_type(inner)?),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        Some(Box::new(Type { range: ty.range(), kind }))
    }

    fn lower_named_type(&mut self, node: Node<'_>) -> Option<Ty> {
        assert_eq!(node.kind(), NodeKind::NAMED_TYPE);
        let kind = TyKind::Named(Name::new(node.text(&self.text)));
        Some(Box::new(Type { range: node.range(), kind }))
    }

    fn lower_list_type(&mut self, node: Node<'_>) -> Option<Ty> {
        assert_eq!(node.kind(), NodeKind::LIST_TYPE);
        let kind = TyKind::List(self.lower_type(node.sole_named_child())?);
        Some(Box::new(Type { range: node.range(), kind }))
    }
}

pub(crate) struct ItemCtxt {
    text: Arc<str>,
    types: Arena<TypeDefinition>,
    directives: Arena<DirectiveDefinition>,
    type_exts: Arena<TypeExtension>,
}

impl ItemCtxt {
    pub(crate) fn new(text: Arc<str>) -> Self {
        Self {
            text,
            types: Default::default(),
            directives: Default::default(),
            type_exts: Default::default(),
        }
    }

    pub fn lower(mut self, tree: Tree) -> Arc<Items> {
        let node = tree.root_node();
        let items = node
            .relevant_children(&mut node.walk())
            .filter_map(|node| self.lower_item(node))
            .collect();
        Arc::new(Items {
            items,
            types: self.types,
            directives: self.directives,
            type_exts: self.type_exts,
        })
    }

    fn lower_item(&mut self, node: Node<'_>) -> Option<Item> {
        assert_eq!(node.kind(), NodeKind::ITEM);
        let def = node.sole_named_child();
        let (name, kind) = match def.kind() {
            NodeKind::TYPE_DEFINITION => {
                let typedef = def.sole_named_child();
                let name_node = match typedef.kind() {
                    NodeKind::OBJECT_TYPE_DEFINITION
                    | NodeKind::INTERFACE_TYPE_DEFINITION
                    | NodeKind::SCALAR_TYPE_DEFINITION
                    | NodeKind::ENUM_TYPE_DEFINITION
                    | NodeKind::UNION_TYPE_DEFINITION
                    | NodeKind::INPUT_OBJECT_TYPE_DEFINITION => typedef.name_node()?,
                    _ =>
                        unreachable!("invalid node kind for type definition: {:?}", typedef.kind()),
                };
                let name = Name::new(name_node.text(&self.text));
                let directives = self.lower_directives_of(typedef);
                (name, ItemKind::TypeDefinition(self.types.alloc(TypeDefinition { directives })))
            }
            NodeKind::TYPE_EXTENSION => {
                let type_ext = def.sole_named_child();
                let name_node = match type_ext.kind() {
                    NodeKind::OBJECT_TYPE_EXTENSION => type_ext.name_node()?,
                    _ => return None,
                };
                let name = Name::new(name_node.text(&self.text));
                let directives = self.lower_directives_of(type_ext);
                (name, ItemKind::TypeExtension(self.type_exts.alloc(TypeExtension { directives })))
            }
            NodeKind::DIRECTIVE_DEFINITION => {
                let name = Name::new(def.name_node()?.text(&self.text));
                (name, ItemKind::DirectiveDefinition(self.directives.alloc(DirectiveDefinition {})))
            }
            // TODO
            _ => return None,
        };
        Some(Item { range: def.range(), name, kind })
    }
}

trait LowerCtxt {
    fn text(&self) -> &str;

    fn name_of(&mut self, node: Node<'_>) -> Option<Name> {
        node.name_node().map(|node| Name::new(node.text(self.text())))
    }

    fn lower_directives_of(&mut self, node: Node<'_>) -> Directives {
        node.child_of_kind(NodeKind::DIRECTIVES)
            .map(|node| self.lower_directives(node))
            .unwrap_or_default()
    }

    fn lower_directives(&mut self, node: Node<'_>) -> Directives {
        assert_eq!(node.kind(), NodeKind::DIRECTIVES);
        node.children_of_kind(&mut node.walk(), NodeKind::DIRECTIVE)
            .filter_map(|node| self.lower_directive(node))
            .collect()
    }

    fn lower_directive(&mut self, node: Node<'_>) -> Option<Directive> {
        assert_eq!(node.kind(), NodeKind::DIRECTIVE);
        // TODO arguments
        let name = Name::new(node.name_node()?.text(self.text()));
        Some(Directive { name })
    }
}

impl LowerCtxt for ItemCtxt {
    fn text(&self) -> &str {
        &self.text
    }
}

impl LowerCtxt for BodyCtxt {
    fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests;
