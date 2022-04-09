#![deny(rust_2018_idioms)]

use std::path::Path;

use self::interner::PathInterner;

mod interner;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(u32);

#[derive(Default)]
pub struct Vfs {
    interner: PathInterner,
}

impl Vfs {
    pub fn intern(&mut self, path: impl AsRef<Path>) -> FileId {
        self.interner.intern(path.as_ref().to_path_buf())
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<FileId> {
        self.interner.get(path.as_ref())
    }

    pub fn lookup(&self, file: FileId) -> &Path {
        self.interner.lookup(file)
    }
}
