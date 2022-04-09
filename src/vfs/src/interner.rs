use std::path::{Path, PathBuf};

use indexmap::IndexSet;

use crate::FileId;

#[derive(Default)]
pub struct PathInterner {
    map: IndexSet<PathBuf>,
}

impl PathInterner {
    pub(crate) fn get(&self, path: &Path) -> Option<FileId> {
        self.map.get_index_of(path).map(|i| FileId(i as u32))
    }

    pub(crate) fn intern(&mut self, path: PathBuf) -> FileId {
        let (id, _added) = self.map.insert_full(path);
        assert!(id < u32::MAX as usize);
        FileId(id as u32)
    }

    pub(crate) fn lookup(&self, id: FileId) -> &Path {
        self.map.get_index(id.0 as usize).unwrap()
    }
}
