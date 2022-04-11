mod db;

use std::collections::HashMap;

pub use db::{DefDatabase, DefDatabaseStorage};

use la_arena::{Arena, Idx};
use smallvec::SmallVec;
use smol_str::SmolStr;
use vfs::VfsPath;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items {
    pub items: Arena<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    TypeDefinition(TypeDefinition),
}

impl Item {
    fn name(&self) -> Name {
        match self {
            Item::TypeDefinition(typedef) => typedef.name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Res {
    path: VfsPath,
    idx: Idx<Item>,
}

#[cfg(test)]
mod tests;
