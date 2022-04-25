use gqls_db::DefDatabase;
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn implementations(&self, file: FileId, at: Point) -> Vec<Location> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
