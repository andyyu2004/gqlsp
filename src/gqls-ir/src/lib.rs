mod body;
mod db;
mod lower;
mod ty;

use std::collections::HashMap;

pub use self::body::*;
pub use self::ty::*;
pub use db::{DefDatabase, DefDatabaseStorage};

use gqls_parse::Range;
use la_arena::{Arena, Idx};
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Item {
    pub range: Range,
    pub kind: ItemKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ItemKind {
    TypeDefinition(Idx<TypeDefinition>),
    DirectiveDefinition(Idx<DirectiveDefinition>),
    TypeExtension(Idx<TypeExtension>),
}

impl Items {
    pub fn name(&self, item: &Item) -> Name {
        match item.kind {
            ItemKind::TypeDefinition(idx) => self.types[idx].name.clone(),
            ItemKind::DirectiveDefinition(idx) => self.directives[idx].name.clone(),
            ItemKind::TypeExtension(idx) => self.type_exts[idx].name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectiveDefinition {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeExtension {
    pub name: Name,
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
// TODO what is the right type for these (should it be something `Range` based or something more like `Res` and index based)
pub type References = Vec<(FileId, Range)>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ItemRes {
    pub file: FileId,
    pub idx: Idx<Item>,
}

#[cfg(test)]
mod tests;
