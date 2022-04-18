use gqls_db::DefDatabase;
use gqls_ir::Res;
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn find_references(&self, file: FileId, at: Point) -> Vec<Location> {
        let res = match self.resolve_item_name_at(file, at)[..] {
            [] => return vec![],
            [res, ..] => res,
        };
        self.references(Res::Item(res))
            .into_iter()
            .map(|(file, range)| Location::new(file, range.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
