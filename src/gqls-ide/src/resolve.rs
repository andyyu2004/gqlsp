use gqls_db::{DefDatabase, SourceDatabase};
use gqls_ir::{FieldRes, ItemRes, ItemResolutions, Name};
use gqls_parse::{NodeExt, NodeKind, RangeExt};
use tree_sitter::Point;
use vfs::FileId;

use crate::Analysis;

impl Analysis {
    pub(crate) fn resolve_item_name_at(&self, file: FileId, at: Point) -> ItemResolutions {
        self.name_at(file, at).map(|name| self.resolve_item(file, name)).unwrap_or_default()
    }

    pub(crate) fn resolve_item_at(&self, file: FileId, at: Point) -> Option<ItemRes> {
        self.item_at(file, at).map(|idx| ItemRes { file, idx })
    }

    pub(crate) fn resolve_field_at(&self, file: FileId, at: Point) -> Option<FieldRes> {
        let item = self.resolve_item_at(file, at)?;
        self.item_body(item)?
            .fields()?
            .iter()
            .find_map(|(idx, field)| field.range.contains(at).then(|| FieldRes { item, idx }))
    }

    pub(crate) fn name_at(&self, file: FileId, at: Point) -> Option<Name> {
        let data = self.file_data(file);
        let root = data.tree.root_node();
        let node = root.named_descendant_for_point_range(at, at)?;
        (node.kind() == NodeKind::NAME).then(|| Name::new(node.text(&data.text)))
    }
}

#[cfg(test)]
mod tests;
