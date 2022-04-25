use gqls_db::DefDatabase;
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn goto_implementation(&self, file: FileId, at: Point) -> Vec<Location> {
        let name = match self.name_at(file, at) {
            Some(name) => name,
            None => return vec![],
        };
        self.implementations(file, name)
            .into_iter()
            .map(|res| self.item(res))
            .map(|item| Location::new(file, item.name.range.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
