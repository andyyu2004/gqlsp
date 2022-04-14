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

    fn resolve_name_at(&self, file: FileId, at: Point) -> Resolutions {
        self.name_at(file, at).map(|name| self.resolve(file, name)).unwrap_or_default()
    }

    fn name_at(&self, file: FileId, at: Point) -> Option<Name> {
        let data = self.file_data(file);
        let root = data.tree.root_node();
        let node = root.named_descendant_for_point_range(at, at)?;
        (node.kind() == NodeKind::NAME).then(|| Name::new(node.text(&data.text)))
    }
}

#[cfg(test)]
mod tests;
