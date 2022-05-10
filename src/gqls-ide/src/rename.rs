use gqls_syntax::Point;
use vfs::FileId;

use crate::{FilePatches, Snapshot};

pub type RenameError = &'static str;

impl Snapshot {
    pub fn prepare_rename(&self, file: FileId, at: Point) -> Result<(), RenameError> {
        Ok(())
    }

    pub fn rename(&self, file: FileId, at: Point) -> Result<Vec<FilePatches>, RenameError> {
        let res = self.resolve_type_at(file, at);
        if res.is_empty() {
            return Err("something");
        }
        Ok(vec![])
    }
}
