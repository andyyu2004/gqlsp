use gqls_db::DefDatabase;
use gqls_syntax::Position;

use crate::{Location, Snapshot};

impl Snapshot {
    pub fn goto_type_definition(&self, position: Position) -> Vec<Location> {
        self.resolve_field_at(position)
            .into_iter()
            .map(|res| (res.item.file, self.field(res)))
            .flat_map(|(file, field)| self.resolve_item(file, field.ty.name()))
            .map(|res| Location::new(res.file, self.item(res).name.range.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
