use std::error::Error;
use std::fmt::{self, Display};

use gqls_syntax::Position;

use crate::{FilePatches, Snapshot};

impl Snapshot {
    // TODO can return a range indicating the rename scope (default behaviour works well enough for now)
    pub fn prepare_rename(&self, position: Position) -> Result<(), RenameError> {
        let res = self.resolve_item_name_at(position);
        if res.is_empty() {
            Err("no references found at position".to_owned())?;
        }
        Ok(())
    }

    pub fn rename(&self, position: Position) -> Result<Vec<FilePatches>, RenameError> {
        self.prepare_rename(position)?;
        Ok(vec![])
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
