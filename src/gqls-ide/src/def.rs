use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::{Name, Resolutions};
use gqls_parse::{NodeExt, NodeKind};
use tree_sitter::Point;
use vfs::VfsPath;

use crate::{Analysis, Location};

impl Analysis {
    pub fn goto_definition(&self, path: VfsPath, at: Point) -> Vec<Location> {
        self.resolve_name_at(path, at)
            .into_iter()
            .map(|res| Location::new(res.path, self.item(res).range.into()))
            .collect()
    }

    fn resolve_name_at(&self, path: VfsPath, at: Point) -> Resolutions {
        self.name_at(path, at).map(|name| self.resolve(name)).unwrap_or_default()
    }

    fn name_at(&self, path: VfsPath, at: Point) -> Option<Name> {
        let data = self.file_data(path);
        let root = data.tree.root_node();
        let node = root.named_descendant_for_point_range(at, at)?;
        (node.kind() == NodeKind::NAME).then(|| Name::new(node.text(&data.text)))
    }
}

#[cfg(test)]
mod tests;
