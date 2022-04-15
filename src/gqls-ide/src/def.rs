use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::{Name, Resolutions};
use gqls_parse::{NodeExt, NodeKind};
use tree_sitter::Point;
use vfs::FileId;

use crate::{Analysis, Location};

impl Analysis {
    pub fn goto_definition(&self, file: FileId, at: Point) -> Vec<Location> {
        self.resolve_name_at(file, at)
            .into_iter()
            .map(|res| Location::new(res.file, self.item(res).range.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
