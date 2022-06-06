use std::fmt::{self, Debug};

use gqls_syntax::{Range, RangeExt};
use la_arena::{Arena, ArenaMap, Idx};

use crate::{ArenaExt, Diagnostic, Directives, Name, Ty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemBody {
    pub diagnostics: Vec<Diagnostic>,
    pub kind: ItemBodyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemBodyKind {
    ObjectTypeDefinition(TypeDefinitionBody),
    InterfaceDefinition(InterfaceDefinitionBody),
    InputObjectTypeDefinition(InputTypeDefinitionBody),
    UnionTypeDefinition(UnionTypeDefinitionBody),
    Todo,
}

impl ItemBody {
    pub fn fields(&self) -> Option<&Arena<Field>> {
        let fields = match &self.kind {
            ItemBodyKind::ObjectTypeDefinition(typedef) => &typedef.fields,
            ItemBodyKind::InputObjectTypeDefinition(typedef) => &typedef.fields,
            ItemBodyKind::InterfaceDefinition(iface) => &iface.fields,
            ItemBodyKind::UnionTypeDefinition(_) => return None,
            ItemBodyKind::Todo => return None,
        };
        Some(&fields.fields)
    }

    pub fn fields_slice(&self) -> Option<&[Field]> {
        self.fields().map(Arena::as_slice)
    }

    pub fn as_union(&self) -> &UnionTypeDefinitionBody {
        if let ItemBodyKind::UnionTypeDefinition(v) = &self.kind {
            v
        } else {
            panic!("expected union typedef")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputTypeDefinitionBody {
    pub fields: Fields,
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

pub type Args = Arena<Arg>;

#[derive(Clone, PartialEq, Eq)]
// FIXME would be nice to have a cleaner representation of args and default
pub struct Field {
    pub range: Range,
    pub name: Name,
    pub ty: Ty,
    pub directives: Directives,
    pub args: Args, // only valid for object fields (empty for input fields)
    pub default: Option<Value>, // only valid for input fields (None for object fields)
}

impl Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Field")
            .field("range", &self.range.debug())
            .field("name", &self.name)
            .field("ty", &self.ty)
            .field("directives", &self.directives)
            .field("arguments", &self.args)
            .field("default", &self.default)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    pub range: Range,
    pub name: Name,
    pub ty: Ty,
    pub default_value: Option<Value>,
    pub directives: Directives,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int,
    Float,
    String,
    Boolean,
    Null,
    Enum,
    List,
    Object,
}
