use gqls_syntax::Position;

use crate::{FilePatches, Snapshot};

pub type RenameError = &'static str;

impl Snapshot {
    pub fn prepare_rename(&self, _position: Position) -> Result<(), RenameError> {
        Ok(())
    }

    pub fn rename(&self, position: Position) -> Result<Vec<FilePatches>, RenameError> {
        let res = self.resolve_type_at(position);
        if res.is_empty() {
            return Err("something");
        }
        Ok(vec![])
    }
}
