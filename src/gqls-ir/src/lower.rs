use std::sync::Arc;

use gqls_parse::{Node, NodeExt, NodeKind};

use crate::{
    Field, Fields, InterfaceDefinitionBody, ItemBody, Name, Ty, TyKind, Type, TypeDefinitionBody
};

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
                ItemBody::TypeDefinition(self.lower_object_typedef(node)),
            NodeKind::INTERFACE_TYPE_DEFINITION =>
                ItemBody::InterfaceDefinition(self.lower_interface_typedef(node)),
            _ => ItemBody::Todo,
        }
    }

    fn lower_object_typedef(&mut self, node: Node<'_>) -> TypeDefinitionBody {
        assert_eq!(node.kind(), NodeKind::OBJECT_TYPE_DEFINITION);
        TypeDefinitionBody { fields: self.lower_fields_of(node) }
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

    fn lower_fields(&mut self, node: Node<'_>) -> Fields {
        assert_eq!(node.kind(), NodeKind::FIELDS_DEFINITION);
        let cursor = &mut node.walk();
        let fields = node
            .children_of_kind(cursor, NodeKind::FIELD_DEFINITION)
            .filter_map(|field| self.lower_field(field));
        Fields::new(fields)
    }

    fn lower_field(&mut self, node: Node<'_>) -> Option<Field> {
        assert_eq!(node.kind(), NodeKind::FIELD_DEFINITION);
        let name = node.name_node()?;
        let ty = self.lower_type(node.child_of_kind(NodeKind::TYPE)?)?;
        Some(Field { range: node.range(), name: Name::new(name.text(&self.text)), ty })
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

#[cfg(test)]
mod tests;
