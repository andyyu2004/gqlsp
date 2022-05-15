use gqls_fixture::{fixture_file, FixtureFile};
use testing::TestDatabaseExt;

use crate::db::TyDatabase;

use super::TestDB;

fn test(fixture_file: &FixtureFile, obj: &str, interface: &str) {
    let db = TestDB::from_fixture_file(fixture_file);
    // db.type_of_item();
    // db.implements_interface();
}

#[test]
fn test_implements_interface() {
    let fixture = fixture_file! {
        "
            interface Bar {}
            type Foo implements Bar {}
        "
    };
    test(&fixture, "Foo", "Bar");
}
