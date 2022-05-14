use std::sync::Arc;

use gqls_base_db::{FileData, SourceDatabase};
use gqls_fixture::{hashmap, Fixture};

pub use {gqls_base_db, gqls_syntax, maplit};

pub trait SourceDatabaseExt {
    fn setup_fixture(&mut self, fixture: &Fixture);
}

impl<DB: SourceDatabase> SourceDatabaseExt for DB {
    fn setup_fixture(&mut self, fixture: &Fixture) {
        self.set_projects(Arc::new(
            hashmap! { "default" => fixture.files().keys().cloned().collect() },
        ));
        for (id, file) in fixture.files() {
            self.set_file_data(id, FileData::new(&file.text, gqls_syntax::parse_fresh(&file.text)));
        }
    }
}
