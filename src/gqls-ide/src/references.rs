use gqls_db::DefDatabase;
use gqls_ir::Res;
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn find_references(&self, file: FileId, at: Point) -> Vec<Location> {
        let res = match self.resolve(file, at) {
            Some(res) => res,
            None => return vec![],
        };
        self.references(res)
            .into_iter()
            .map(|(file, range)| Location::new(file, range.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
