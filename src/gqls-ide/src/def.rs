use gqls_db::SourceDatabase;
use gqls_parse::{NodeExt, NodeKind};
use tree_sitter::Point;
use vfs::VfsPath;

use crate::{Analysis, Range};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct DefinitionRange {
    pub path: VfsPath,
    pub range: Range,
}

impl Analysis {
    pub fn find_definition(&self, path: VfsPath, at: Point) -> Option<DefinitionRange> {
        let tree = self.snapshot.file_tree(path);
        let node = tree.root_node().named_descendant_for_point_range(at, at)?;
        match node.kind() {
            NodeKind::NAME => todo!(),
            _ => return None,
        }
    }
}

#[cfg(test)]
mod tests;
