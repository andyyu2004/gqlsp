use std::sync::Arc;

use crate::{DefDatabase, DefDatabaseStorage, Item, Items, Name, TypeDefinition};
use gqls_base_db::{FileData, SourceDatabase, SourceDatabaseStorage};
use la_arena::Arena;
use vfs::Vfs;

#[salsa::database(SourceDatabaseStorage, DefDatabaseStorage)]
#[derive(Default)]
pub(crate) struct TestDB {
    storage: salsa::Storage<TestDB>,
}

impl salsa::Database for TestDB {
}

#[test]
fn test_definitions() {
    let mut db = TestDB::default();
    let mut vfs = Vfs::default();
    let path = vfs.intern("foo");

    let gql = r#"
        type Foo {
           bar: Bar
        }

        type Bar {
            foo: Foo
        }
    "#;
    db.set_file_data(path, FileData::new(gql, gqls_parse::parse_fresh(gql)));
    let items = db.items(path);
    assert_eq!(
        *items,
        Items {
            items: Arena::from_iter([
                Item::TypeDefinition(TypeDefinition { name: Name::new("Foo") }),
                Item::TypeDefinition(TypeDefinition { name: Name::new("Bar") })
            ])
        }
    );
}
