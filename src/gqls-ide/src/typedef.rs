use gqls_db::DefDatabase;
use gqls_ir::InProject;
use gqls_syntax::Position;

use crate::{Location, Snapshot};

impl Snapshot {
    pub fn goto_type_definition(&self, position: Position) -> Vec<Location> {
        match self.resolve_field_at(position) {
            Some(res) => {
                let field = self.field(res);
                self.resolve_type(InProject::new(position.file, field.ty))
                    .into_iter()
                    .map(|res| Location::new(res.file, self.item(res).name.range))
                    .collect()
            }
            None => self.goto_definition(position),
        }
    }
}

#[cfg(test)]
mod tests;
