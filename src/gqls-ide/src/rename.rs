use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};

use gqls_db::DefDatabase;
use gqls_syntax::Position;
use vfs::FileId;

use crate::{FilePatches, Patch, Range, Snapshot};

impl Snapshot {
    // TODO can return a range indicating the rename scope (default behaviour works well enough for now)
    pub fn prepare_rename(&self, position: Position) -> Result<Range, RenameError> {
        if self.resolve_item_name_at(position).is_empty() {
            Err("no references found at position".to_owned())?;
        }
        let name = self.name_at(position).expect("there were references here so it must be a name");
        Ok(name.range.into())
    }

    pub fn rename(&self, position: Position, to: &str) -> Result<Vec<FilePatches>, RenameError> {
        self.prepare_rename(position)?;
        let mut patches = HashMap::<FileId, FilePatches>::new();
        self.find_references(position).into_iter().for_each(|location| {
            patches
                .entry(location.file)
                .or_insert_with(|| FilePatches::empty(location.file))
                .patches
                .push(Patch::new(location.range, to.to_owned()));
        });
        assert!(!patches.is_empty());
        Ok(patches.into_values().collect())
    }
}

#[derive(Debug)]
pub struct RenameError(String);

impl From<String> for RenameError {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Display for RenameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for RenameError {}

#[cfg(test)]
mod tests;
