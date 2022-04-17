use gqls_parse::Range;
use la_arena::{Arena, ArenaMap, Idx};

use crate::{Name, Ty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemBody {
    ObjectTypeDefinition(TypeDefinitionBody),
    ObjectTypeExtension(TypeExtensionBody),
    InterfaceDefinition(InterfaceDefinitionBody),
    InputObjectTypeDefinition(InputTypeDefinitionBody),
    Todo,
}

impl ItemBody {
    pub fn fields(&self) -> Option<&Arena<Field>> {
        match self {
            ItemBody::ObjectTypeDefinition(typedef) => Some(&typedef.fields.fields),
            ItemBody::ObjectTypeExtension(type_ext) => Some(&type_ext.fields.fields),
            ItemBody::InputObjectTypeDefinition(typedef) => Some(&typedef.fields.fields),
            ItemBody::InterfaceDefinition(iface) => Some(&iface.fields.fields),
            ItemBody::Todo => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputTypeDefinitionBody {
    pub fields: InputFields,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinitionBody {
    pub fields: Fields,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeExtensionBody {
    pub fields: Fields,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDefinitionBody {
    pub fields: Fields,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Fields {
    pub fields: Arena<Field>,
}

impl Fields {
    pub fn new(fields: impl IntoIterator<Item = Field>) -> Self {
        Self { fields: fields.into_iter().collect() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub range: Range,
    pub name: Name,
    pub ty: Ty,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InputFields {
    pub fields: Arena<Field>,
    pub default_values: ArenaMap<Idx<Field>, ()>,
}

impl InputFields {
    pub fn new(
        fields: impl IntoIterator<Item = Field>,
        default_values: ArenaMap<Idx<Field>, ()>,
    ) -> Self {
        Self { fields: fields.into_iter().collect(), default_values }
    }
}
