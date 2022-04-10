use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::VfsPath;

#[derive(Debug, Default)]
pub struct PathInterner {
    map: HashSet<&'static Path>,
}

impl PathInterner {
    pub(crate) fn get(&self, path: &Path) -> Option<VfsPath> {
        self.map.get(path).copied()
    }

    pub(crate) fn intern(&mut self, path: PathBuf) -> VfsPath {
        match self.map.get(path.as_path()) {
            Some(interned) => interned,
            None => {
                let path = Box::leak(path.into_boxed_path());
                self.map.insert(path);
                path
            }
        }
    }
}
