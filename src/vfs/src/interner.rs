use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub type PathInterner = Interner<Path>;

#[derive(Debug)]
pub struct Interner<T: ?Sized + 'static> {
    map: HashSet<&'static T>,
}

impl<T: ?Sized> Default for Interner<T> {
    fn default() -> Self {
        Self { map: Default::default() }
    }
}

pub trait IntoBoxed {
    type Unowned: ToOwned<Owned = Self> + ?Sized;
    fn into_boxed(self) -> Box<Self::Unowned>;
}

impl IntoBoxed for PathBuf {
    type Unowned = Path;

    fn into_boxed(self) -> Box<Path> {
        self.into_boxed_path()
    }
}

impl IntoBoxed for String {
    type Unowned = str;

    fn into_boxed(self) -> Box<str> {
        self.into_boxed_str()
    }
}

impl<T> Interner<T>
where
    T: ToOwned + Hash + Eq + ?Sized,
    T::Owned: Hash + Eq + Deref<Target = T> + IntoBoxed<Unowned = T>,
{
    pub fn get(&self, path: &T) -> Option<&'static T> {
        self.map.get(path).copied()
    }

    pub fn intern(&mut self, item: T::Owned) -> &'static T {
        match self.map.get(&*item) {
            Some(interned) => interned,
            None => {
                let path = Box::leak(item.into_boxed());
                self.map.insert(path);
                path
            }
        }
    }
}
