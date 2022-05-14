use crate::db::TyDatabase;
use crate::TyDatabaseStorage;
use expect_test::{expect, Expect};
use gqls_fixture::{fixture, Fixture};
use gqls_ir::{DefDatabase, DefDatabaseStorage, Name, SourceDatabaseStorage};
use testing::{file_id, TestDatabaseExt};

#[salsa::database(SourceDatabaseStorage, DefDatabaseStorage, TyDatabaseStorage)]
#[derive(Default)]
pub(crate) struct TestDB {
    storage: salsa::Storage<TestDB>,
}

impl salsa::Database for TestDB {
}

fn test_type_of(fixture: &Fixture, name: &str, expect: Expect) {
    let db = TestDB::from_fixture(&fixture);
    let resolutions = db.resolve_item(file_id!(""), Name::unranged(name));
    assert_eq!(resolutions.len(), 1);
    let ty = db.type_of(resolutions[0]);
    expect.assert_debug_eq(&ty);
}

fn test_field_types_of(fixture: &Fixture, name: &str, expect: Expect) {
    let db = TestDB::from_fixture(&fixture);
    let resolutions = db.resolve_item(file_id!(""), Name::unranged(name));
    assert_eq!(resolutions.len(), 1);
    let fields = db.field_types_of(resolutions[0]);
    expect.assert_debug_eq(&fields);
}

#[test]
fn test_typeof_simple_object() {
    let fixture = fixture! {
        "" => "
            type Foo {
                id: ID!
            }
        "
    };

    test_type_of(
        &fixture,
        "Foo",
        expect![[r#"
            object Foo
        "#]],
    );

    test_field_types_of(
        &fixture,
        "Foo",
        expect![[r#"
        {
          id
        }
    "#]],
    );
}
