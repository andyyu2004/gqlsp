use gqls_db::DefDatabase;
use gqls_ir::Res;
use gqls_syntax::Position;

use crate::{Location, Snapshot};

impl Snapshot {
    pub fn find_references(&self, position: Position) -> Vec<Location> {
        let res = match self.resolve_item_name_at(position).and_then(|res| res.try_into_item().ok())
        {
            Some(res) => res,
            None => return vec![],
        };
        self.references(Res::Item(res))
            .into_iter()
            .map(|(file, range)| Location::new(file, range))
            .collect()
    }
}

#[cfg(test)]
mod tests;
