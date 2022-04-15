use gqls_db::DefDatabase;
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn find_references(&self, file: FileId, at: Point) -> Vec<Location> {
        let name = match self.name_at(file, at) {
            Some(name) => name,
            None => return vec![],
        };
        self.references(file, name)
            .into_iter()
            .map(|(file, range)| Location::new(file, range.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
