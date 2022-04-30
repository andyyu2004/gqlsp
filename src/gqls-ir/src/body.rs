use std::fmt::{self, Debug};

use gqls_syntax::Range;
use la_arena::{Arena, ArenaMap, Idx};

use crate::{ArenaExt, Directives, Name, RangeDebug, Ty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemBody {
    ObjectTypeDefinition(TypeDefinitionBody),
    InterfaceDefinition(InterfaceDefinitionBody),
    InputObjectTypeDefinition(InputTypeDefinitionBody),
    UnionTypeDefinition(UnionTypeDefinitionBody),
    Todo,
}

impl ItemBody {
    pub fn fields(&self) -> Option<&Arena<Field>> {
        let fields = match self {
            ItemBody::ObjectTypeDefinition(typedef) => &typedef.fields.fields,
            ItemBody::InputObjectTypeDefinition(typedef) => &typedef.fields.fields,
            ItemBody::InterfaceDefinition(iface) => &iface.fields.fields,
            ItemBody::UnionTypeDefinition(_) => return None,
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
pub struct UnionTypeDefinitionBody {
    pub types: Vec<Ty>,
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

#[derive(Clone, PartialEq, Eq)]
pub struct Field {
    pub range: Range,
    pub name: Name,
    pub ty: Ty,
    pub directives: Directives,
}

impl Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Field")
            .field("range", &RangeDebug(self.range))
            .field("name", &self.name)
            .field("ty", &self.ty)
            .field("directives", &self.directives)
            .finish()
    }
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
