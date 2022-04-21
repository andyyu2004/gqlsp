use gqls_db::DefDatabase;
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn goto_definition(&self, file: FileId, at: Point) -> Vec<Location> {
        self.resolve_item_name_at(file, at)
            .into_iter()
            .map(|res| Location::new(res.file, self.item(res).name.range.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
