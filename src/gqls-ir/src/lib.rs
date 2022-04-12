mod db;

use std::collections::HashMap;

pub use db::{DefDatabase, DefDatabaseStorage};

use gqls_parse::Range;
use la_arena::{Arena, Idx};
use smallvec::SmallVec;
use smol_str::SmolStr;
use vfs::VfsPath;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items {
    pub items: Arena<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub range: Range,
    pub kind: ItemKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemKind {
    TypeDefinition(TypeDefinition),
    TypeExtension(TypeExtension),
}

impl Item {
    fn name(&self) -> Name {
        match &self.kind {
            ItemKind::TypeDefinition(typedef) => typedef.name.clone(),
            ItemKind::TypeExtension(ext) => ext.name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeExtension {
    pub name: Name,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Name(SmolStr);

impl Name {
    pub fn new(s: &str) -> Self {
        Self(SmolStr::new(s))
    }
}

pub type ItemMap = HashMap<Name, SmallVec<[Idx<Item>; 1]>>;
pub type Resolutions = SmallVec<[Res; 1]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Res {
    pub path: VfsPath,
    pub idx: Idx<Item>,
}

impl PartialOrd for Res {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Res {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path).then_with(|| self.idx.into_raw().cmp(&other.idx.into_raw()))
    }
}

#[cfg(test)]
mod tests;
