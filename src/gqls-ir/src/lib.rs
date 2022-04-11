mod db;

pub use db::{DefDatabase, DefDatabaseStorage};

use la_arena::Arena;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items {
    pub items: Arena<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    TypeDefinition(TypeDefinition),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(SmolStr);

impl Name {
    pub fn new(s: &str) -> Self {
        Self(SmolStr::new(s))
    }
}

#[cfg(test)]
mod tests;
