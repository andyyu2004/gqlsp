use gqls_db::DefDatabase;
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn find_references(&self, file: FileId, at: Point) -> Vec<Location> {
        let name = self.name_at(file, at);
        todo!()
    }
}

#[cfg(test)]
mod tests;
