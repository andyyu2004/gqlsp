use gqls_db::DefDatabase;
use gqls_syntax::Position;

use crate::{Location, Snapshot};

impl Snapshot {
    pub fn goto_implementation(&self, position: Position) -> Vec<Location> {
        let name = match self.name_at(position) {
            Some(name) => name,
            None => return vec![],
        };
        self.implementations(position.file, name)
            .into_iter()
            .map(|res| self.item(res))
            .map(|item| Location::new(position.file, item.name.range))
            .collect()
    }
}

#[cfg(test)]
mod tests;
