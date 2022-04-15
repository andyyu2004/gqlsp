use la_arena::Arena;

use crate::{Name, Ty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemBody {
    TypeDefinition(TypeDefinitionBody),
    Todo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinitionBody {
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
    pub name: Name,
    pub ty: Ty,
}
