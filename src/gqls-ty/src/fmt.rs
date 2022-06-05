use itertools::Itertools;
use std::fmt::{self, Debug};

use crate::{FieldType, FieldTypes, InterfaceType, Interned, ObjectType, TyKind, Type};

impl Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Debug for TyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyKind::Boolean => write!(f, "Boolean"),
            TyKind::Float => write!(f, "Float"),
            TyKind::ID => write!(f, "ID"),
            TyKind::Int => write!(f, "Int"),
            TyKind::String => write!(f, "String"),
            TyKind::Object(obj) => obj.fmt(f),
            TyKind::Input(..) => todo!(),
            TyKind::Interface(..) => todo!(),
            TyKind::NonNull(inner) => write!(f, "{inner:?}!"),
            TyKind::List(inner) => write!(f, "[{inner:?}]"),
            TyKind::Err => write!(f, "<err>"),
            TyKind::Union(union) => write!(f, "{:?}", union.types.iter().format(" | ")),
            TyKind::Enum(_) => todo!(),
            TyKind::Scalar(_) => todo!(),
        }
    }
}

impl Debug for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object {}", self.name)
    }
}

impl Debug for InterfaceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "interface {}", self.name)
    }
}

impl Debug for FieldTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for field in &self.fields {
            writeln!(f, "  {:?}", field)?;
        }
        write!(f, "}}")
    }
}

impl Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: Debug> Debug for Interned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
