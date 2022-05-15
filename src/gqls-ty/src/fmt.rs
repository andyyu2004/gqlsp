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
            Self::Boolean => write!(f, "Boolean"),
            Self::Float => write!(f, "Float"),
            Self::ID => write!(f, "ID"),
            Self::Int => write!(f, "Int"),
            Self::String => write!(f, "String"),
            Self::Object(obj) => obj.fmt(f),
            Self::Input(..) => todo!(),
            Self::Interface(..) => todo!(),
            Self::NonNull(inner) => write!(f, "{inner:?}!"),
            Self::List(inner) => write!(f, "[{inner:?}]"),
            TyKind::Err => write!(f, "<err>"),
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
