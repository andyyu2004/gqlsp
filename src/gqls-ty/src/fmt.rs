use std::fmt::{self, Debug};

use crate::{FieldType, FieldTypes, ObjectType, TyKind, Type};

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
            Self::Id => write!(f, "Id"),
            Self::Int => write!(f, "Int"),
            Self::String => write!(f, "String"),
            Self::Object(obj) => obj.fmt(f),
            Self::Input(..) => todo!(),
            Self::Interface(..) => todo!(),
            Self::NonNull(inner) => write!(f, "{inner:?}!"),
            Self::List(inner) => write!(f, "[{inner:?}]"),
        }
    }
}

impl Debug for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object {}", self.name)
    }
}

impl Debug for FieldTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for field in &self.fields {
            writeln!(f, "  {}", field.name)?;
        }
        write!(f, "}}")
    }
}
