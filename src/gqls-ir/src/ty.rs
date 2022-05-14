use crate::{Name, Range};
use std::fmt::{self, Debug};

pub type Ty = Box<Type>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Type {
    pub range: Range,
    pub kind: TyKind,
}

impl Type {
    pub fn new_named(name: Name) -> Ty {
        Box::new(Self { range: name.range, kind: TyKind::Named(name) })
    }

    pub fn name(&self) -> Name {
        match &self.kind {
            TyKind::Named(name) => name.clone(),
            TyKind::NonNull(ty) | TyKind::List(ty) => ty.name(),
        }
    }

    pub fn is_builtin(&self) -> bool {
        matches!(self.name().as_str(), "Int" | "Float" | "String" | "Boolean" | "ID")
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TyKind {
    Named(Name),
    NonNull(Ty),
    List(Ty),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scalar {
    Id,
    Int,
    String,
    Float,
    Custom(CustomScalar),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomScalar {}

impl Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.kind, f)
    }
}

impl Debug for TyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TyKind::Named(name) => write!(f, "{}", name.name),
            TyKind::NonNull(ty) => write!(f, "{ty:?}!"),
            TyKind::List(ty) => write!(f, "[{ty:?}]"),
        }
    }
}
