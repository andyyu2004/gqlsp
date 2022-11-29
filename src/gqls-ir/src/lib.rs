#![deny(rust_2018_idioms)]

mod body;
mod db;
mod diagnostic;
mod lower;
mod ty;

pub use self::body::*;
pub use self::db::{DefDatabase, DefDatabaseStorage};
pub use self::diagnostic::{Diagnostic, DiagnosticKind};
pub use self::ty::*;
pub use gqls_base_db::{InFile, InProject, SourceDatabase, SourceDatabaseStorage};
pub use la_arena::{Arena, Idx, IdxRange, RawIdx};

use gqls_syntax::{Node, NodeExt, Point, Range, RangeExt};
use smallvec::SmallVec;
use smol_str::SmolStr;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;
use vfs::FileId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items {
    pub items: Arena<Item>,
    typedefs: Arena<TypeDefinition>,
    directives: Arena<DirectiveDefinition>,
}

macro_rules! impl_index {
    (Idx<$ty:ty> for $self:ty: $field:ident) => {
        impl<'ir> std::ops::Index<Idx<$ty>> for $self {
            type Output = $ty;

            fn index(&self, index: la_arena::Idx<$ty>) -> &Self::Output {
                &self.$field[index]
            }
        }
    };
}

impl_index!(Idx<TypeDefinition> for Items: typedefs);
impl_index!(Idx<DirectiveDefinition> for Items: directives);
impl_index!(Idx<Item> for Items: items);

impl Items {
    pub fn directives(&self, idx: Idx<Item>) -> Option<&Directives> {
        match self.items[idx].kind {
            ItemKind::TypeDefinition(typedef) => Some(&self.typedefs[typedef].directives),
            ItemKind::DirectiveDefinition(_) => None,
        }
    }

    pub fn implements(&self, idx: Idx<Item>, interface: &Name) -> bool {
        match self.items[idx].kind {
            ItemKind::TypeDefinition(typedef) => &self.typedefs[typedef].implementations,
            ItemKind::DirectiveDefinition(_) => return false,
        }
        .as_ref()
        .map(|implements| implements.contains(interface))
        .unwrap_or_default()
    }
}

impl Deref for Items {
    type Target = Arena<Item>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Item {
    pub name: Name,
    pub range: Range,
    pub kind: ItemKind,
}

impl Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Item")
            .field("name", &self.name)
            .field("range", &self.range.debug())
            .field("kind", &self.kind)
            .finish()
    }
}

pub type Directives = Vec<Directive>;

#[derive(Clone, PartialEq, Eq)]
pub struct Directive {
    pub range: Range,
    pub name: Name,
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct DirectiveLocations: u16 {
        const ARGUMENT_DEFINITION = 1 << 0;
        const ENUM = 1 << 1;
        const ENUM_VALUE = 1 << 2;
        const FIELD_DEFINITION = 1 << 3;
        const INPUT_FIELD_DEFINITION = 1 << 4;
        const INPUT_OBJECT = 1 << 5;
        const INTERFACE = 1 << 6;
        const OBJECT = 1 << 7;
        const SCALAR = 1 << 8;
        const SCHEMA = 1 << 9;
        const UNION = 1 << 10;
    }
}

impl Debug for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ItemKind {
    TypeDefinition(Idx<TypeDefinition>),
    DirectiveDefinition(Idx<DirectiveDefinition>),
}

impl ItemKind {
    pub fn into_type_definition(self) -> Idx<TypeDefinition> {
        if let Self::TypeDefinition(v) = self { v } else { panic!() }
    }
}

pub type Implementations = HashSet<Name>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    pub directives: Directives,
    pub implementations: Option<Implementations>,
    pub kind: TypeDefinitionKind,
    pub is_ext: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDefinitionKind {
    Object,
    Interface,
    Input,
    Scalar,
    Enum,
    Union,
}

impl TypeDefinitionKind {
    pub fn desc(&self) -> &'static str {
        match self {
            Self::Object => "object",
            Self::Interface => "interface",
            Self::Input => "input",
            Self::Scalar => "scalar",
            Self::Enum => "enum",
            Self::Union => "union",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectiveDefinition {
    pub locations: DirectiveLocations,
}

#[derive(Clone, Ord, PartialOrd, Eq)]
pub struct Name {
    name: SmolStr,
    pub range: Range,
}

impl Borrow<SmolStr> for Name {
    fn borrow(&self) -> &SmolStr {
        &self.name
    }
}

impl Name {
    pub fn name(&self) -> SmolStr {
        self.name.clone()
    }
}

impl Deref for Name {
    type Target = SmolStr;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name, f)
    }
}

impl Name {
    pub fn new(text: &(impl HasText + ?Sized), node: Node<'_>) -> Self {
        Self { name: SmolStr::new(node.text(text.text())), range: node.range() }
    }

    pub fn unranged(s: &str) -> Self {
        Self {
            name: SmolStr::new(s),
            range: Range {
                start_byte: 0,
                end_byte: 0,
                start_point: Point::new(0, 0),
                end_point: Point::new(0, 0),
            },
        }
    }
}

pub trait HasText {
    fn text(&self) -> &str;
    fn text_of(&self, node: Node<'_>) -> &str {
        node.text(self.text())
    }
}

impl HasText for Arc<str> {
    fn text(&self) -> &str {
        self
    }
}

impl HasText for str {
    fn text(&self) -> &str {
        self
    }
}

pub type ProjectItems = HashMap<FileId, Arc<Items>>;
pub type ItemMap = HashMap<Name, SmallVec<[Idx<Item>; 1]>>;
pub type ItemResolutions = SmallVec<[ItemRes; 1]>;
// TODO what is the right type for these (should it be something `Range` based or something more like `Res` and index based)
pub type References = Vec<(FileId, Range)>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Res {
    Builtin(Builtin),
    // INVARIANT: should be non-empty
    Item(ItemResolutions),
    Err,
}

impl Res {
    pub fn into_item(self) -> ItemResolutions {
        if let Self::Item(v) = self { v } else { panic!("expected item resolution") }
    }

    pub fn try_into_item(self) -> Result<ItemResolutions, Self> {
        if let Self::Item(v) = self { Ok(v) } else { Err(self) }
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Builtin {
    Scalar(BuiltinScalar),
    Directive(BuiltinDirective),
}

impl From<BuiltinScalar> for Builtin {
    fn from(v: BuiltinScalar) -> Self {
        Self::Scalar(v)
    }
}

impl From<BuiltinDirective> for Builtin {
    fn from(v: BuiltinDirective) -> Self {
        Self::Directive(v)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BuiltinDirective {
    Deprecated,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BuiltinScalar {
    Boolean,
    Float,
    ID,
    Int,
    String,
}

pub type ItemRes = InFile<Idx<Item>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FieldRes {
    pub item: ItemRes,
    pub idx: Idx<Field>,
}

impl FieldRes {
    pub fn new(item: ItemRes, idx: Idx<Field>) -> Self {
        Self { item, idx }
    }
}

trait ArenaExt {
    type Item;
    fn as_slice(&self) -> &[Self::Item];
}

impl<T> ArenaExt for Arena<T> {
    type Item = T;

    // An abuse of the api to obtain a full slice of the arena
    fn as_slice(&self) -> &[Self::Item] {
        fn make<T>(i: u32) -> Idx<T> {
            Idx::from_raw(RawIdx::from(i))
        }
        &self[IdxRange::new(make(0)..make(self.len() as u32))]
    }
}

#[cfg(test)]
mod tests;
