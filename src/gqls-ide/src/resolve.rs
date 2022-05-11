use gqls_db::DefDatabase;
use gqls_ir::{FieldRes, ItemRes, ItemResolutions};
use gqls_syntax::{Position, RangeExt};

use crate::Snapshot;

// TODO resolve logic should be moved to the ir layer
impl Snapshot {
    pub(crate) fn resolve_item_name_at(&self, position: Position) -> ItemResolutions {
        self.name_at(position)
            .map(|name| self.resolve_item(position.file, name))
            .unwrap_or_default()
    }

    pub(crate) fn resolve_type_at(&self, position: Position) -> ItemResolutions {
        self.type_at(position)
            .map(|ty| self.resolve_item(position.file, ty.name()))
            .unwrap_or_default()
    }

    pub(crate) fn resolve_item_at(&self, position: Position) -> Option<ItemRes> {
        self.item_at(position).map(|idx| ItemRes { file: position.file, idx })
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
