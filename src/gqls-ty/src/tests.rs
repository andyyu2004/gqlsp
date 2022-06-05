mod implements_interface;

use crate::db::TyDatabase;
use crate::TyDatabaseStorage;
use expect_test::{expect, Expect};
use gqls_fixture::{fixture_file, FixtureFile};
use gqls_ir::{DefDatabase, DefDatabaseStorage, InProject, Name, SourceDatabaseStorage};
use testing::{file_id, TestDatabaseExt};

#[salsa::database(SourceDatabaseStorage, DefDatabaseStorage, TyDatabaseStorage)]
#[derive(Default)]
pub(crate) struct TestDB {
    storage: salsa::Storage<TestDB>,
}

impl salsa::Database for TestDB {
}

fn test_type_of_item(fixture: &FixtureFile, name: &str, expect: Expect) {
    let db = TestDB::from_fixture_file(fixture);
    let resolutions =
        db.resolve_item(InProject::new(file_id!(""), Name::unranged(name))).into_item();
    assert_eq!(resolutions.len(), 1);
    let ty = db.type_of_item(resolutions[0]);
    expect.assert_debug_eq(&ty);
}

fn test_field_types_of(fixture: &FixtureFile, name: &str, expect: Expect) {
    let db = TestDB::from_fixture_file(&fixture);
    let resolutions =
        db.resolve_item(InProject::new(file_id!(""), Name::unranged(name))).into_item();
    assert_eq!(resolutions.len(), 1);
    let fields = db.field_types_of(resolutions[0]);
    let mut s = String::new();
    s.push_str("{\n");
    for field in fields.fields {
        s.push_str(&format!("  {}: {:?}\n", field.name, db.type_of_field(field.res)));
    }
    s.push('}');
    expect.assert_eq(&s);
}

fn test_typeof_extended_object() {
}

#[test]
fn test_typeof_simple_object() {
    let fixture = fixture_file! {
        "
            type Foo {
                id: ID!
            }
        "
    };

    test_type_of_item(
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
              id: ID!
            }"#]],
    );
}

#[test]
fn test_typeof_recursive_object() {
    return;
    let fixture = fixture_file! {
        "
            type Foo {
                id: ID!
                foo: [Foo]
            }
        "
    };

    test_type_of_item(
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
                id: Id!
            }"#]],
    );
}
