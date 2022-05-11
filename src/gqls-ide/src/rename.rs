use std::error::Error;
use std::fmt::{self, Display};

use gqls_db::DefDatabase;
use gqls_syntax::Position;

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
        let patches = self
            .find_references(position)
            .into_iter()
            .map(|location| FilePatches {
                file: location.file,
                patches: vec![Patch { range: location.range, with: to.to_owned() }],
            })
            .collect::<Vec<_>>();
        assert!(!patches.is_empty());
        Ok(patches)
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
