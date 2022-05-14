use std::sync::Arc;

use gqls_base_db::{FileData, SourceDatabase};
use gqls_fixture::{hashmap, Fixture};

pub use {gqls_base_db, gqls_syntax, maplit};

#[macro_export]
macro_rules! setup_db {
    ($db:ident: {
        $($file:ident: $text:expr,)*
     }) => {
        use $crate::gqls_base_db::{FileData, SourceDatabase};
        $db.set_projects(std::sync::Arc::new(
            $crate::maplit::hashmap! { "default" => $crate::maplit::hashset! { $($file),* } },
        ));
        $(
            $db.set_file_data($file, FileData::new($text, $crate::gqls_syntax::parse_fresh($text)));
        )*
    };
}

pub fn setup_db(db: &mut dyn SourceDatabase, fixture: &Fixture) {
    db.set_projects(Arc::new(hashmap! { "default" => fixture.files().keys().cloned().collect() }));
    for (id, file) in fixture.files() {
        db.set_file_data(id, FileData::new(&file.text, gqls_syntax::parse_fresh(&file.text)));
    }
}
