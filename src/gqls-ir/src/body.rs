use gqls_syntax::Range;
use itertools::Itertools;
use std::fmt::{self, Debug};

use la_arena::Arena;

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

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Fields {
    pub fields: Arena<Field>,
}

impl Debug for Fields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for (_, field) in self.fields.iter() {
            writeln!(f, "  {field:?}")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Fields {
    pub fn new(fields: impl IntoIterator<Item = Field>) -> Self {
        Self { fields: fields.into_iter().collect() }
    }
}

pub type Args = Vec<Arg>;

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
        let Self { name, ty, directives, args, default, .. } = self;
        write!(f, "{name}")?;
        if !args.is_empty() {
            write!(f, "({:?})", args.iter().format(", "))?;
        }
        write!(f, ": {ty:?}")?;
        if let Some(default) = default {
            write!(f, " = {default:?}")?;
        }
        if !directives.is_empty() {
            write!(f, " {:?}", directives.iter().format(" "))?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Arg {
    pub range: Range,
    pub name: Name,
    pub ty: Ty,
    pub default_value: Option<Value>,
    pub directives: Directives,
}

impl Debug for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, ty, default_value, directives, .. } = self;
        write!(f, "{name}: {ty:?}")?;
        if let Some(value) = default_value {
            write!(f, "= {value:?}")?;
        }

        if !directives.is_empty() {
            write!(f, " {:?}", directives.iter().format(" "))?;
        }

        Ok(())
    }
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
