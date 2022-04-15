use gqls_db::DefDatabase;
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn find_references(&self, file: FileId, at: Point) -> Vec<Location> {
        self.resolve_name_at(file, at)
            .into_iter()
            .map(|res| Location::new(res.file, self.item(res).range.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
