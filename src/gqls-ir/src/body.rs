use gqls_parse::Range;
use la_arena::{Arena, ArenaMap, Idx};

use crate::{ArenaExt, Directives, Name, Ty};

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
        let fields = match self {
            ItemBody::ObjectTypeDefinition(typedef) => &typedef.fields.fields,
            ItemBody::ObjectTypeExtension(type_ext) => &type_ext.fields.fields,
            ItemBody::InputObjectTypeDefinition(typedef) => &typedef.fields.fields,
            ItemBody::InterfaceDefinition(iface) => &iface.fields.fields,
            ItemBody::Todo => return None,
        };
        Some(fields)
    }

    pub fn fields_slice(&self) -> Option<&[Field]> {
        self.fields().map(Arena::as_slice)
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
    pub directives: Directives,
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
