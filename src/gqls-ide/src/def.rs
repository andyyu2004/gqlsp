use gqls_db::DefDatabase;
use gqls_syntax::Position;

use crate::{Location, Snapshot};

impl Snapshot {
    pub fn goto_definition(&self, position: Position) -> Vec<Location> {
        self.resolve_item_name_at(position)
            .into_iter()
            .flat_map(|res| res.try_into_item().ok())
            .flatten()
            .map(|res| Location::new(res.file, self.item(res).name.range))
            .collect()
    }
}

#[cfg(test)]
mod tests;
