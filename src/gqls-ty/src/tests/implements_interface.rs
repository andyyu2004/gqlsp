use gqls_fixture::{fixture_file, FixtureFile};
use testing::TestDatabaseExt;



use super::TestDB;

fn test(fixture_file: &FixtureFile, _obj: &str, _interface: &str) {
    let _db = TestDB::from_fixture_file(fixture_file);
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
