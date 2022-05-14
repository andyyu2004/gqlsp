pub use db::TyDatabaseStorage;

mod db;
mod fmt;

use gqls_ir::FieldRes;
use smol_str::SmolStr;
use std::sync::Arc;

pub type Ty = Arc<Type>;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Type {
    pub kind: TyKind,
}

#[derive(PartialEq, Eq, Clone, Hash)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FieldType {
    name: SmolStr,
    res: FieldRes,
}

impl TyKind {
    fn intern(self) -> Ty {
        Ty::new(Type { kind: self })
    }
}

#[cfg(test)]
mod tests;
