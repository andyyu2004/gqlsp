mod has_type;
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

fn test_type_of_item(fixture: &FixtureFile, item_name: &str, expect: Expect) {
    let db = TestDB::from_fixture_file(fixture);
    let resolutions =
        db.resolve_item(InProject::new(file_id!(""), Name::unranged(item_name))).into_item();
    assert_eq!(resolutions.len(), 1);
    let ty = db.type_of_item(resolutions[0]);
    expect.assert_debug_eq(&ty);
}

fn test_field_types_of(fixture: &FixtureFile, name: &str, expect: Expect) {
    let db = TestDB::from_fixture_file(fixture);
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

#[test]
fn test_typeof_extended_object() {
}

#[test]
fn test_typeof_union() {
    let fixture = fixture_file! {
        "
            type Foo {
                id: ID!
            }

            type Bar {
                id: ID!
            }

            union Union = Foo | Bar
        "
    };
    test_type_of_item(
        &fixture,
        "Union",
        expect![[r#"
            union Union(object Foo | object Bar)
        "#]],
    )
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
    let fixture = fixture_file! {
        "
            type Foo {
                id: ID!
                foo: [Foo!]
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
              foo: [object Foo!]
            }"#]],
    );
}

#[macro_export]
macro_rules! val {
    ([ $($tt:tt),* ]) => {
        gqls_ir::Value::List(::std::sync::Arc::from([ $(val!($tt)),* ]))
    };
    ({ $($key:ident: $value:tt),* }) => {
        gqls_ir::Value::Object(::std::sync::Arc::from(maplit::btreemap! {
            $(
                gqls_ir::Name::unranged(stringify!($key)) => val!($value),
            )*
        }))
    };
    (null) => {
        gqls_ir::Value::Null
    };
    (false) => {
        gqls_ir::Value::Boolean(false)
    };
    (true) => {
        gqls_ir::Value::Boolean(true)
    };
    ($variant:ident) => {
        gqls_ir::Value::Enum(stringify!($variant).into())
    };
    ($lit:literal) => {{ gqls_ir::Value::from($lit) }};
}

#[macro_export]
macro_rules! ty {
    // unclear how to do object type due to the indirection in fields
    ([ $($tt:tt)* ]) => {
        $crate::TyKind::List(ty!($($tt)*)).intern()
    };
    (!$($tt:tt)*) => {
        $crate::TyKind::NonNull(ty!($($tt)*)).intern()
    };
    (Boolean) => {
        $crate::TyKind::Boolean.intern()
    };
    (Float) => {
        $crate::TyKind::Float.intern()
    };
    (String) => {
        $crate::TyKind::String.intern()
    };
    (Int) => {
        $crate::TyKind::Int.intern()
    };
    (ID) => {
        $crate::TyKind::ID.intern()
    };
    ($($variant:ident)|+) => {
        $crate::TyKind::Enum($crate::EnumType {
            name: "".into(),
            variants: ::std::sync::Arc::from([
                $(stringify!($variant).into()),+
            ])
        }).intern()
    };
}
