use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;
use std::path::Path;

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

impl<T> Interner<T>
where
    T: ToOwned + Hash + Eq + ?Sized,
    T::Owned: Hash + Eq + Deref<Target = T>,
{
    pub(crate) fn get(&self, path: &T) -> Option<&'static T> {
        self.map.get(path).copied()
    }

    pub(crate) fn intern(&mut self, item: T::Owned) -> &'static T {
        match self.map.get(&*item) {
            Some(interned) => interned,
            None => {
                let path = Box::leak(Box::new(item));
                self.map.insert(path);
                path
            }
        }
    }
}
