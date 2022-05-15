use std::sync::Arc;

use gqls_base_db::{FileData, SourceDatabase};
use gqls_fixture::{hashmap, Fixture, FixtureFile};

pub use {gqls_base_db, gqls_syntax, maplit};

pub trait TestDatabaseExt: Default + SourceDatabase {
    fn setup_fixture(&mut self, fixture: &Fixture);
    fn from_fixture(fixture: &Fixture) -> Self;
    fn from_fixture_file(fixture_file: &FixtureFile) -> Self;
}

impl<DB> TestDatabaseExt for DB
where
    DB: SourceDatabase + Default,
{
    fn setup_fixture(&mut self, fixture: &Fixture) {
        self.set_projects(Arc::new(
            hashmap! { "default" => fixture.files().keys().cloned().collect() },
        ));
        for (id, file) in fixture.files() {
            self.set_file_data(id, FileData::new(&file.text, gqls_syntax::parse_fresh(&file.text)));
        }
    }

    fn from_fixture(fixture: &Fixture) -> Self {
        let mut db = Self::default();
        db.setup_fixture(fixture);
        db
    }

    fn from_fixture_file(fixture_file: &FixtureFile) -> Self {
        Self::from_fixture(&Fixture::single(fixture_file.clone()))
    }
}

#[macro_export]
macro_rules! file_id {
    ($file:literal) => {
        ::std::path::Path::new($file)
    };
}
