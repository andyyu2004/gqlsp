pub use db::{TyDatabase, TyDatabaseStorage};

mod db;
mod fmt;

use gqls_ir::FieldRes;
use smol_str::SmolStr;
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

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Type {
    pub kind: TyKind,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum TyKind {
    Boolean,
    Float,
    ID,
    Int,
    String,
    Err,
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

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct FieldType {
    name: SmolStr,
    res: FieldRes,
}

impl TyKind {
    fn intern(self) -> Ty {
        Ty::new(Type { kind: self })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImplError;

#[cfg(test)]
mod tests;
