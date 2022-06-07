pub use db::{TyDatabase, TyDatabaseStorage};

mod db;
mod fmt;

use gqls_ir::{BuiltinScalar, FieldRes};
use smol_str::SmolStr;
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;

pub type Ty = Interned<Type>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Interned<T>(Arc<T>);

impl<T> Deref for Interned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Interned<T> {
    pub fn new(value: T) -> Self {
        Interned(Arc::new(value))
    }
}

bitflags::bitflags! {
    pub struct TypeFlags: u8 {
        const HAS_ERROR = 1 << 0;
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Type {
    pub kind: TyKind,
    flags: TypeFlags,
}

impl Type {
    pub fn desc(&self) -> Cow<'static, str> {
        self.kind.desc()
    }

    pub fn is_input(&self) -> bool {
        match &self.kind {
            TyKind::Object(_) | TyKind::Interface(_) | TyKind::Union(_) => false,
            TyKind::NonNull(ty) | TyKind::List(ty) => ty.is_input(),
            _ => true,
        }
    }

    pub fn is_output(&self) -> bool {
        match &self.kind {
            TyKind::Input(_) => false,
            TyKind::NonNull(ty) | TyKind::List(ty) => ty.is_output(),
            _ => true,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum TyKind {
    Boolean,
    Float,
    ID,
    Int,
    String,
    Err,
    Union(UnionType),
    Enum(EnumType),
    Scalar(ScalarType),
    Object(ObjectType),
    Input(InputObjectType),
    Interface(InterfaceType),
    NonNull(Ty),
    List(Ty),
}

impl TyKind {
    pub fn desc(&self) -> Cow<'static, str> {
        let s = match self {
            TyKind::Boolean | TyKind::Float | TyKind::ID | TyKind::Int | TyKind::String =>
                "builtin scalar",
            TyKind::Scalar(_) => "scalar",
            TyKind::Err => "<err>",
            TyKind::Union(_) => "union",
            TyKind::Enum(_) => "enum",
            TyKind::Object(_) => "object",
            TyKind::Input(_) => "input object",
            TyKind::Interface(_) => "interface",
            TyKind::NonNull(ty) => return Cow::Owned(format!("non-null {}", ty.desc())),
            TyKind::List(ty) => return Cow::Owned(format!("list of {}s", ty.desc())),
        };
        Cow::Borrowed(s)
    }
}

impl From<BuiltinScalar> for TyKind {
    fn from(scalar: BuiltinScalar) -> Self {
        match scalar {
            BuiltinScalar::Boolean => TyKind::Boolean,
            BuiltinScalar::Float => TyKind::Float,
            BuiltinScalar::ID => TyKind::ID,
            BuiltinScalar::Int => TyKind::Int,
            BuiltinScalar::String => TyKind::String,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct UnionType {
    name: SmolStr,
    types: Vec<Ty>,
}

impl UnionType {
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct ScalarType {
    name: SmolStr,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct EnumType {
    name: SmolStr,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct FieldTypes {
    pub fields: Vec<FieldType>,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct ObjectType {
    name: SmolStr,
    fields: FieldTypes,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct InputObjectType {
    name: SmolStr,
    fields: FieldTypes,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct InterfaceType {
    name: SmolStr,
    fields: FieldTypes,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct FieldType {
    name: SmolStr,
    res: FieldRes,
}

impl TyKind {
    fn intern(self) -> Ty {
        let flags = self.type_flags();
        Ty::new(Type { kind: self, flags })
    }

    fn type_flags(&self) -> TypeFlags {
        match self {
            TyKind::Err => TypeFlags::HAS_ERROR,
            TyKind::Enum(_)
            | TyKind::Scalar(_)
            | TyKind::Boolean
            | TyKind::Float
            | TyKind::ID
            | TyKind::Int
            | TyKind::String => TypeFlags::empty(),
            TyKind::NonNull(ty) | TyKind::List(ty) => ty.flags,
            TyKind::Union(union) =>
                union.types.iter().fold(TypeFlags::empty(), |flags, ty| flags | ty.flags),
            TyKind::Object(ObjectType { fields, .. })
            | TyKind::Interface(InterfaceType { fields, .. })
            | TyKind::Input(InputObjectType { fields, .. }) => {
                let _ = fields;
                // TODO unclear how to handle this
                TypeFlags::empty()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImplError;

#[cfg(test)]
mod tests;
