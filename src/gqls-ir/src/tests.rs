use std::path::Path;
use std::sync::Arc;

use crate::{DefDatabase, DefDatabaseStorage, Item, Items, Name, Res, TypeDefinition};
use gqls_base_db::{FileData, SourceDatabase, SourceDatabaseStorage};
use la_arena::Arena;
use maplit::hashmap;
use smallvec::smallvec;
use vfs::Vfs;

#[salsa::database(SourceDatabaseStorage, DefDatabaseStorage)]
#[derive(Default)]
pub(crate) struct TestDB {
    storage: salsa::Storage<TestDB>,
}

impl salsa::Database for TestDB {
}

macro_rules! idx {
    ($idx:expr) => {
        la_arena::Idx::from_raw(la_arena::RawIdx::from($idx))
    };
}

#[test]
fn test_definitions() {
    let mut db = TestDB::default();
    let mut vfs = Vfs::default();

    let foo = vfs.intern("foo");
    let foogql = r#"
        type Foo {
           bar: Bar
        }

        type Foo {
            foo: Foo
        }

        type Bar {
            foo: Foo
        }
    "#;

    let bar = vfs.intern("bar");
    let bargql = r#"
        type Bar {
            baz: Baz
        }

        type Baz {
            foo: Foo
        }
    "#;
    db.set_files(Arc::new(vec![foo, bar]));
    db.set_file_data(foo, FileData::new(foogql, gqls_parse::parse_fresh(foogql)));
    db.set_file_data(bar, FileData::new(bargql, gqls_parse::parse_fresh(bargql)));

    let items = db.items(foo);
    assert_eq!(
        *items,
        Items {
            items: Arena::from_iter([
                Item::TypeDefinition(TypeDefinition { name: Name::new("Foo") }),
                Item::TypeDefinition(TypeDefinition { name: Name::new("Foo") }),
                Item::TypeDefinition(TypeDefinition { name: Name::new("Bar") })
            ])
        }
    );

    let item_map = db.item_map(foo);
    assert_eq!(
        *item_map,
        hashmap! {
            Name::new("Foo") => smallvec![idx!(0), idx!(1)],
            Name::new("Bar") => smallvec![idx!(2)],
        }
    );

    let resolutions = db.resolve(Name::new("Foo"));
    assert_eq!(
        resolutions.as_slice(),
        [
            Res { path: Path::new("foo"), idx: idx!(0) },
            Res { path: Path::new("foo"), idx: idx!(1) }
        ]
    );

    let resolutions = db.resolve(Name::new("Bar"));
    assert_eq!(
        resolutions.as_slice(),
        [
            Res { path: Path::new("foo"), idx: idx!(2) },
            Res { path: Path::new("bar"), idx: idx!(0) }
        ]
    );
}
