pub use db::TyDatabaseStorage;

mod db;

use smol_str::SmolStr;
use std::sync::Arc;

pub type Ty = Arc<Type>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Type {
    pub kind: TyKind,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TyKind {
    Boolean,
    Float,
    Id,
    Int,
    String,
    Object(ObjectType),
    Input(InputObjectType),
    Interface(InterfaceType),
    NonNull(Ty),
    List(Ty),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Fields {
    pub fields: Vec<FieldType>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ObjectType {
    fields: Fields,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct InputObjectType {
    fields: Fields,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct InterfaceType {
    fields: Fields,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FieldType {
    name: SmolStr,
    ty: Ty,
}

impl TyKind {
    fn intern(self) -> Ty {
        Ty::new(Type { kind: self })
    }
}

#[cfg(test)]
mod tests;
