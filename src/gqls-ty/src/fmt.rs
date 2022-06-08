use crate::*;
use itertools::Itertools;
use std::fmt::{self, Debug, Display};

impl Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Debug for TyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyKind::Boolean | TyKind::Float | TyKind::ID | TyKind::Int | TyKind::String =>
                write!(f, "{self}"),
            TyKind::Object(obj) => write!(f, "{obj:?}"),
            TyKind::Input(input) => write!(f, "{input:?}"),
            TyKind::Interface(interface) => write!(f, "{interface:?}"),
            TyKind::NonNull(inner) => write!(f, "{inner:?}!"),
            TyKind::List(inner) => write!(f, "[{inner:?}]"),
            TyKind::Union(union) => write!(f, "{union:?}"),
            TyKind::Enum(e) => write!(f, "{e:?}"),
            TyKind::Scalar(scalar) => write!(f, "{scalar:?}"),
            TyKind::Err => write!(f, "<err>"),
        }
    }
}

impl Debug for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object {}", self.name)
    }
}

impl Debug for InputObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "input {}", self.name)
    }
}

impl Debug for UnionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "union {}({:?})", self.name, self.types.iter().format(" | "))
    }
}

impl Debug for InterfaceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "interface {}", self.name)
    }
}

impl Debug for EnumType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enum {}({})", self.name, self.variants.iter().format(" | "))
    }
}

impl Debug for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar {}", self.name)
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

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for TyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyKind::Boolean => write!(f, "Boolean"),
            TyKind::Float => write!(f, "Float"),
            TyKind::ID => write!(f, "ID"),
            TyKind::Int => write!(f, "Int"),
            TyKind::String => write!(f, "String"),
            TyKind::Object(obj) => Display::fmt(obj, f),
            TyKind::Input(input) => Display::fmt(input, f),
            TyKind::Interface(interface) => Display::fmt(interface, f),
            TyKind::NonNull(inner) => write!(f, "{inner}!"),
            TyKind::List(inner) => write!(f, "[{inner}]"),
            TyKind::Err => write!(f, "<err>"),
            TyKind::Union(union) => Display::fmt(union, f),
            TyKind::Enum(e) => Display::fmt(e, f),
            TyKind::Scalar(scalar) => Display::fmt(scalar, f),
        }
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for UnionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for InputObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for EnumType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for InterfaceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: Display> Display for Interned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for TypeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeMismatch::InvalidVariant(variant, e) =>
                write!(f, "`{variant}` is not a valid variant of enum `{}`", e.name),
            TypeMismatch::InvalidNullField(name) =>
                write!(f, "non-nullable field `{}` must be provided", name),
            TypeMismatch::ExtraneousField(name, ty) => {
                write!(f, "field `{name}` is not a member of type `{ty}`")
            }
            TypeMismatch::InvalidNull => write!(f, "expected non-nullable value"),
            TypeMismatch::Obvious(value, ty) =>
                write!(f, "cannot use {} value as {} type", value.desc(), ty.desc()),
        }
    }
}
