use gqls_parse::Range;
use la_arena::Arena;

use crate::{Name, Ty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemBody {
    TypeDefinition(TypeDefinitionBody),
    InterfaceDefinition(InterfaceDefinitionBody),
    Todo,
}

impl ItemBody {
    pub fn fields(&self) -> Option<&Fields> {
        match self {
            ItemBody::TypeDefinition(typedef) => Some(&typedef.fields),
            ItemBody::InterfaceDefinition(iface) => Some(&iface.fields),
            ItemBody::Todo => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinitionBody {
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
