#![deny(rust_2018_idioms)]

use std::path::Path;

use self::interner::PathInterner;

mod interner;

pub type VfsPath = &'static Path;

#[derive(Default)]
pub struct Vfs {
    interner: PathInterner,
}

impl Vfs {
    pub fn intern(&mut self, path: impl AsRef<Path>) -> VfsPath {
        self.interner.intern(path.as_ref().to_path_buf())
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<VfsPath> {
        self.interner.get(path.as_ref())
    }
}
