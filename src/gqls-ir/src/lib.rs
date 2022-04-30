#![deny(rust_2018_idioms)]

mod body;
mod db;
mod lower;
mod ty;

pub use self::body::*;
pub use self::ty::*;
pub use db::{DefDatabase, DefDatabaseStorage};
pub use la_arena::{Arena, Idx, RawIdx};

use gqls_syntax::{Node, NodeExt, Range};
use la_arena::IdxRange;
use smallvec::SmallVec;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use vfs::FileId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items {
    pub items: Arena<Item>,
    pub typedefs: Arena<TypeDefinition>,
    pub directives: Arena<DirectiveDefinition>,
}

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

#[derive(Clone, PartialEq, Eq)]
pub struct Item {
    pub name: Name,
    pub range: Range,
    pub kind: ItemKind,
}

struct RangeDebug(Range);

impl Debug for RangeDebug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Range { start_point, end_point, .. } = self.0;
        write!(
            f,
            "{}:{}..{}:{}",
            start_point.row, start_point.column, end_point.row, end_point.column
        )
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Item")
            .field("name", &self.name)
            .field("range", &RangeDebug(self.range))
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

impl Debug for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Directive")
            .field("range", &RangeDebug(self.range))
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ItemKind {
    TypeDefinition(Idx<TypeDefinition>),
    DirectiveDefinition(Idx<DirectiveDefinition>),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectiveDefinition {}

#[derive(Clone, Eq)]
pub struct Name {
    pub range: Range,
    name: SmolStr,
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
    pub fn new(t: &(impl HasText + ?Sized), node: Node<'_>) -> Self {
        Self { name: SmolStr::new(node.text(t.text())), range: node.range() }
    }

    pub fn unranged(s: &str) -> Self {
        use gqls_syntax::Point;

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
