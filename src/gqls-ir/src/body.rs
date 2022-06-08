use gqls_syntax::Range;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display};
use std::sync::Arc;

use la_arena::Arena;

use crate::{ArenaExt, Diagnostic, Directives, Name, Ty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemBody {
    pub diagnostics: Vec<Diagnostic>,
    pub kind: ItemBodyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemBodyKind {
    Enum(EnumDefinitionBody),
    Interface(InterfaceDefinitionBody),
    InputObject(InputTypeDefinitionBody),
    Object(ObjectTypeDefinitionBody),
    Union(UnionDefinitionBody),
    Todo,
}

impl ItemBody {
    pub fn fields(&self) -> Option<&Arena<Field>> {
        let fields = match &self.kind {
            ItemBodyKind::Object(typedef) => &typedef.fields,
            ItemBodyKind::InputObject(typedef) => &typedef.fields,
            ItemBodyKind::Interface(iface) => &iface.fields,
            ItemBodyKind::Enum(_) | ItemBodyKind::Union(_) | ItemBodyKind::Todo => return None,
        };
        Some(&fields.fields)
    }

    pub fn fields_slice(&self) -> Option<&[Field]> {
        self.fields().map(Arena::as_slice)
    }

    pub fn as_union(&self) -> &UnionDefinitionBody {
        if let ItemBodyKind::Union(v) = &self.kind { v } else { panic!("expected union typedef") }
    }

    pub fn as_enum(&self) -> &EnumDefinitionBody {
        if let ItemBodyKind::Enum(v) = &self.kind { v } else { panic!("expected enum typedef") }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDefinitionBody {
    pub variants: Variants,
}

pub type Variants = Vec<Variant>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub name: Name,
}

impl Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputTypeDefinitionBody {
    pub fields: Fields,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionDefinitionBody {
    pub types: Vec<Ty>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectTypeDefinitionBody {
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
    pub default_value: Option<Value>, // only valid for input fields (None for object fields)
}

impl Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, ty, directives, args, default_value, .. } = self;
        write!(f, "{name}")?;
        if !args.is_empty() {
            write!(f, "({:?})", args.iter().format(", "))?;
        }
        write!(f, ": {ty:?}")?;
        if let Some(default) = default_value {
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
            write!(f, " = {value:?}")?;
        }

        if !directives.is_empty() {
            write!(f, " {:?}", directives.iter().format(" "))?;
        }

        Ok(())
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Value {
    // storing as u64 to avoid f64 Eq pain
    Boolean(bool),
    Enum(Arc<str>),
    Float(u64),
    Int(i32),
    List(Arc<[Value]>),
    Null,
    Object(Arc<BTreeMap<Name, Value>>),
    String(Arc<str>),
}

impl Value {
    pub fn desc(&self) -> &'static str {
        match self {
            Value::Boolean(_) => "boolean",
            Value::Enum(_) => "enum",
            Value::Float(_) => "float",
            Value::Int(_) => "integer",
            Value::List(_) => "list",
            Value::Null => "null",
            Value::Object(_) => "object",
            Value::String(_) => "string",
        }
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Int(v)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f.to_bits())
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::String(s.into())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(u) => write!(f, "{}", f64::from_bits(*u)),
            Self::String(s) => write!(f, "{s:?}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Enum(s) => write!(f, "{s}"),
            Self::List(vs) => write!(f, "[{:?}]", vs.iter().format(", ")),
            Self::Object(vs) =>
                write!(f, "{{ {} }}", vs.iter().map(|(k, v)| format!("{k}: {v:?}")).format(", ")),
            Self::Null => write!(f, "null"),
        }
    }
}
