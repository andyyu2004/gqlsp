use gqls_db::DefDatabase;
use gqls_ir::{FieldRes, InProject, ItemRes, Res};
use gqls_syntax::{Position, RangeExt};

use crate::Snapshot;

// TODO resolve logic should be moved to the ir layer
impl Snapshot {
    pub(crate) fn resolve_item_name_at(&self, position: Position) -> Option<Res> {
        self.name_at(position).map(|name| self.resolve_item(InProject::new(position.file, name)))
    }

    pub(crate) fn resolve_item_at(&self, position: Position) -> Option<ItemRes> {
        self.item_at(position).map(|idx| ItemRes::new(position.file, idx))
    }

    pub(crate) fn resolve_field_at(&self, position: Position) -> Option<FieldRes> {
        let item = self.resolve_item_at(position)?;
        self.item_body(item)?.fields()?.iter().find_map(|(idx, field)| {
            field.range.contains(position.point).then(|| FieldRes { item, idx })
        })
    }
}

#[cfg(test)]
mod tests;
