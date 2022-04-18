#![deny(rust_2018_idioms)]

mod body;
mod db;
mod lower;
mod ty;

use std::collections::HashMap;

pub use self::body::*;
pub use self::ty::*;
pub use db::{DefDatabase, DefDatabaseStorage};
pub use la_arena::{Arena, Idx, RawIdx};

use gqls_parse::Range;
use smallvec::SmallVec;
use smol_str::SmolStr;
use std::fmt::{self, Debug, Display};
use vfs::FileId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items {
    pub items: Arena<Item>,
    pub types: Arena<TypeDefinition>,
    pub directives: Arena<DirectiveDefinition>,
    pub type_exts: Arena<TypeExtension>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub name: Name,
    pub range: Range,
    pub kind: ItemKind,
}

pub type Directives = Vec<Directive>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directive {
    pub name: Name,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ItemKind {
    TypeDefinition(Idx<TypeDefinition>),
    DirectiveDefinition(Idx<DirectiveDefinition>),
    TypeExtension(Idx<TypeExtension>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    directives: Directives,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectiveDefinition {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeExtension {
    directives: Directives,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Name(SmolStr);

impl Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Name {
    pub fn new(s: &str) -> Self {
        Self(SmolStr::new(s))
    }
}

pub type ItemMap = HashMap<Name, SmallVec<[Idx<Item>; 1]>>;
pub type ItemResolutions = SmallVec<[ItemRes; 1]>;
pub type Resolutions = SmallVec<[Res; 1]>;
// TODO what is the right type for these (should it be something `Range` based or something more like `Res` and index based)
pub type References = Vec<(FileId, Range)>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Res {
    Item(ItemRes),
    Field(FieldRes),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ItemRes {
    pub file: FileId,
    pub idx: Idx<Item>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FieldRes {
    pub item: ItemRes,
    pub idx: Idx<Field>,
}

#[cfg(test)]
mod tests;
