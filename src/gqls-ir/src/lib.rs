mod db;

pub use db::{DefDatabase, DefDatabaseStorage};

use la_arena::Arena;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definitions {
    defs: Arena<TypeDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {}

#[cfg(test)]
mod tests;
