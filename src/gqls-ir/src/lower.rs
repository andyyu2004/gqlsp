use std::sync::Arc;

use gqls_syntax::{Node, NodeExt, NodeKind, Tree};
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
            NodeKind::OBJECT_TYPE_DEFINITION | NodeKind::OBJECT_TYPE_EXTENSION =>
                ItemBody::ObjectTypeDefinition(self.lower_object_typedef(node)),
            NodeKind::INTERFACE_TYPE_DEFINITION =>
                ItemBody::InterfaceDefinition(self.lower_interface_typedef(node)),
            NodeKind::INPUT_OBJECT_TYPE_DEFINITION =>
                ItemBody::InputObjectTypeDefinition(self.lower_input_object_typedef(node)),
            NodeKind::UNION_TYPE_DEFINITION =>
                ItemBody::UnionTypeDefinition(self.lower_union_typedef(node)),
            // TODO enum etc
            _ => ItemBody::Todo,
        }
    }

    fn lower_object_typedef(&mut self, node: Node<'_>) -> TypeDefinitionBody {
        assert!(
            [NodeKind::OBJECT_TYPE_DEFINITION, NodeKind::OBJECT_TYPE_EXTENSION]
                .contains(&node.kind())
        );
        TypeDefinitionBody { fields: self.lower_fields_of(node) }
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

    fn lower_union_typedef(&mut self, node: Node<'_>) -> UnionTypeDefinitionBody {
        assert_eq!(node.kind(), NodeKind::UNION_TYPE_DEFINITION);
        let types = node
            .child_of_kind(NodeKind::UNION_MEMBER_TYPES)
            .map(|node| self.lower_union_member_types(node))
            .unwrap_or_default();
        UnionTypeDefinitionBody { types }
    }

    fn lower_union_member_types(&mut self, node: Node<'_>) -> Vec<Ty> {
        assert_eq!(node.kind(), NodeKind::UNION_MEMBER_TYPES);
        node.children_of_kind(&mut node.walk(), NodeKind::NAMED_TYPE)
            .map(|node| self.lower_named_type(node))
            .collect()
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
}

pub(crate) struct ItemCtxt {
    text: Arc<str>,
    typedefs: Arena<TypeDefinition>,
    directives: Arena<DirectiveDefinition>,
}

impl ItemCtxt {
    pub(crate) fn new(text: Arc<str>) -> Self {
        Self { text, typedefs: Default::default(), directives: Default::default() }
    }

    pub fn lower(mut self, tree: Tree) -> Arc<Items> {
        let node = tree.root_node();
        let items = node
            .relevant_children(&mut node.walk())
            .filter_map(|node| self.lower_item(node))
            .collect();
        Arc::new(Items { items, typedefs: self.typedefs, directives: self.directives })
    }

    fn lower_item(&mut self, node: Node<'_>) -> Option<Item> {
        assert_eq!(node.kind(), NodeKind::ITEM);
        let def = node.sole_named_child()?;
        let (name, kind) = match def.kind() {
            NodeKind::TYPE_DEFINITION => {
                let typedef = def.sole_named_child()?;
                let kind = match typedef.kind() {
                    NodeKind::OBJECT_TYPE_DEFINITION => TypeDefinitionKind::Object,
                    NodeKind::INTERFACE_TYPE_DEFINITION => TypeDefinitionKind::Interface,
                    NodeKind::SCALAR_TYPE_DEFINITION => TypeDefinitionKind::Scalar,
                    NodeKind::ENUM_TYPE_DEFINITION => TypeDefinitionKind::Enum,
                    NodeKind::UNION_TYPE_DEFINITION => TypeDefinitionKind::Union,
                    NodeKind::INPUT_OBJECT_TYPE_DEFINITION => TypeDefinitionKind::Input,
                    _ =>
                        unreachable!("invalid node kind for type definition: {:?}", typedef.kind()),
                };
                let name_node = typedef.name_node()?;
                let name = Name::new(self, name_node);
                let directives = self.lower_directives_of(typedef);
                let implementations = self.try_lower_implementations_of(typedef);
                (
                    name,
                    ItemKind::TypeDefinition(self.typedefs.alloc(TypeDefinition {
                        is_ext: false,
                        kind,
                        directives,
                        implementations,
                    })),
                )
            }
            NodeKind::TYPE_EXTENSION => {
                let type_ext = def.sole_named_child()?;
                let kind = match type_ext.kind() {
                    NodeKind::OBJECT_TYPE_EXTENSION => TypeDefinitionKind::Object,
                    // TODO
                    _ => return None,
                };
                let name_node = type_ext.name_node()?;
                let name = Name::new(self, name_node);
                let directives = self.lower_directives_of(type_ext);
                let implementations = self.try_lower_implementations_of(type_ext);
                (
                    name,
                    ItemKind::TypeDefinition(self.typedefs.alloc(TypeDefinition {
                        is_ext: true,
                        kind,
                        directives,
                        implementations,
                    })),
                )
            }
            NodeKind::DIRECTIVE_DEFINITION => {
                let name = Name::new(self, def.name_node()?);
                let locations_node = def.child_of_kind(NodeKind::DIRECTIVE_LOCATIONS)?;
                let locations = locations_node
                    .children_of_kind(&mut locations_node.walk(), NodeKind::DIRECTIVE_LOCATION)
                    .map(|child| match child.text(self.text()) {
                        "ARGUMENT_DEFINITION" => DirectiveLocations::ARGUMENT_DEFINITION,
                        "ENUM" => DirectiveLocations::ENUM,
                        "ENUM_VALUE" => DirectiveLocations::ENUM_VALUE,
                        "FIELD_DEFINITION" => DirectiveLocations::FIELD_DEFINITION,
                        "INPUT_FIELD_DEFINITION" => DirectiveLocations::INPUT_FIELD_DEFINITION,
                        "INPUT_OBJECT" => DirectiveLocations::INPUT_OBJECT,
                        "INTERFACE" => DirectiveLocations::INTERFACE,
                        "OBJECT" => DirectiveLocations::OBJECT,
                        "SCALAR" => DirectiveLocations::SCALAR,
                        "SCHEMA" => DirectiveLocations::SCHEMA,
                        "UNION" => DirectiveLocations::UNION,
                        location => unreachable!("found invalid directive location: {location}",),
                    })
                    .fold(DirectiveLocations::default(), |acc, location| acc | location);
                (
                    name,
                    ItemKind::DirectiveDefinition(
                        self.directives.alloc(DirectiveDefinition { locations }),
                    ),
                )
            }
            // TODO
            _ => return None,
        };
        Some(Item { range: def.range(), name, kind })
    }

    fn try_lower_implementations_of(&mut self, node: Node<'_>) -> Option<Implementations> {
        let implementations = node.child_of_kind(NodeKind::IMPLEMENTS_INTERFACES)?;
        let cursor = &mut implementations.walk();
        Some(
            implementations
                .children_of_kind(cursor, NodeKind::NAMED_TYPE)
                .map(|node| self.lower_named_type(node).name())
                .collect(),
        )
    }
}

pub(crate) trait LowerCtxt: HasText {
    fn name_of(&mut self, node: Node<'_>) -> Option<Name> {
        node.name_node().map(|node| Name::new(self, node))
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
        let name = Name::new(self, node.name_node()?);
        Some(Directive { range: node.range(), name })
    }

    fn lower_type(&mut self, node: Node<'_>) -> Option<Ty> {
        assert!(matches!(
            node.kind(),
            NodeKind::TYPE | NodeKind::NAMED_TYPE | NodeKind::LIST_TYPE | NodeKind::NON_NULL_TYPE
        ));
        let ty =
            if matches!(node.kind(), NodeKind::TYPE) { node.sole_named_child()? } else { node };
        let kind = match ty.kind() {
            NodeKind::NAMED_TYPE => return Some(self.lower_named_type(ty)),
            NodeKind::LIST_TYPE => TyKind::List(self.lower_type(ty.sole_named_child()?)?),
            NodeKind::NON_NULL_TYPE => {
                let inner = ty.sole_named_child()?;
                match inner.kind() {
                    NodeKind::NAMED_TYPE => TyKind::NonNull(self.lower_named_type(inner)),
                    NodeKind::LIST_TYPE => TyKind::NonNull(self.lower_list_type(inner)?),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        Some(Box::new(Type { range: ty.range(), kind }))
    }

    fn lower_list_type(&mut self, node: Node<'_>) -> Option<Ty> {
        assert_eq!(node.kind(), NodeKind::LIST_TYPE);
        let kind = TyKind::List(self.lower_type(node.sole_named_child()?)?);
        Some(Box::new(Type { range: node.range(), kind }))
    }

    fn lower_named_type(&mut self, node: Node<'_>) -> Ty {
        assert_eq!(node.kind(), NodeKind::NAMED_TYPE);
        let kind = TyKind::Named(Name::new(self, node));
        Box::new(Type { range: node.range(), kind })
    }
}

impl<C: HasText> LowerCtxt for C {
}

impl HasText for ItemCtxt {
    fn text(&self) -> &str {
        &self.text
    }
}

impl HasText for BodyCtxt {
    fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests;
