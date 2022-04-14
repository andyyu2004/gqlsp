use std::path::Path;
use std::sync::Arc;

use crate::{DefDatabase, DefDatabaseStorage, Name, Res};
use expect_test::expect;
use gqls_base_db::{FileData, SourceDatabase, SourceDatabaseStorage};
use maplit::{hashmap, hashset};
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

        extend type Bar {
            i: Int!
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

        directive @d on FIELD
    "#;
    db.set_projects(Arc::new(hashmap! { "project" => hashset!{ foo, bar } }));
    db.set_file_data(foo, FileData::new(foogql, gqls_parse::parse_fresh(foogql)));
    db.set_file_data(bar, FileData::new(bargql, gqls_parse::parse_fresh(bargql)));

    let item_map = db.item_map(foo);
    assert_eq!(
        *item_map,
        hashmap! {
            Name::new("Foo") => smallvec![idx!(0), idx!(1)],
            Name::new("Bar") => smallvec![idx!(2), idx!(3)],
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

    let mut resolutions = db.resolve(Name::new("Bar"));
    resolutions.sort();
    assert_eq!(
        resolutions.as_slice(),
        [
            Res { path: Path::new("bar"), idx: idx!(0) },
            Res { path: Path::new("foo"), idx: idx!(2) },
            Res { path: Path::new("foo"), idx: idx!(3) },
        ]
    );

    let resolutions = db.resolve(Name::new("d"));
    assert_eq!(resolutions.as_slice(), [Res { path: Path::new("bar"), idx: idx!(2) },]);

    let items = db.items(foo);
    expect![[r#"
        Items {
            items: Arena {
                len: 4,
                data: [
                    Item {
                        range: Range {
                            start_byte: 9,
                            end_byte: 49,
                            start_point: Point {
                                row: 1,
                                column: 8,
                            },
                            end_point: Point {
                                row: 3,
                                column: 9,
                            },
                        },
                        kind: TypeDefinition(
                            TypeDefinition {
                                name: Name(
                                    "Foo",
                                ),
                            },
                        ),
                    },
                    Item {
                        range: Range {
                            start_byte: 59,
                            end_byte: 100,
                            start_point: Point {
                                row: 5,
                                column: 8,
                            },
                            end_point: Point {
                                row: 7,
                                column: 9,
                            },
                        },
                        kind: TypeDefinition(
                            TypeDefinition {
                                name: Name(
                                    "Foo",
                                ),
                            },
                        ),
                    },
                    Item {
                        range: Range {
                            start_byte: 110,
                            end_byte: 151,
                            start_point: Point {
                                row: 9,
                                column: 8,
                            },
                            end_point: Point {
                                row: 11,
                                column: 9,
                            },
                        },
                        kind: TypeDefinition(
                            TypeDefinition {
                                name: Name(
                                    "Bar",
                                ),
                            },
                        ),
                    },
                    Item {
                        range: Range {
                            start_byte: 161,
                            end_byte: 208,
                            start_point: Point {
                                row: 13,
                                column: 8,
                            },
                            end_point: Point {
                                row: 15,
                                column: 9,
                            },
                        },
                        kind: TypeExtension(
                            TypeExtension {
                                name: Name(
                                    "Bar",
                                ),
                            },
                        ),
                    },
                ],
            },
        }
    "#]]
    .assert_debug_eq(&items);

    let items = db.items(bar);
    expect![[r#"
        Items {
            items: Arena {
                len: 3,
                data: [
                    Item {
                        range: Range {
                            start_byte: 9,
                            end_byte: 50,
                            start_point: Point {
                                row: 1,
                                column: 8,
                            },
                            end_point: Point {
                                row: 3,
                                column: 9,
                            },
                        },
                        kind: TypeDefinition(
                            TypeDefinition {
                                name: Name(
                                    "Bar",
                                ),
                            },
                        ),
                    },
                    Item {
                        range: Range {
                            start_byte: 60,
                            end_byte: 101,
                            start_point: Point {
                                row: 5,
                                column: 8,
                            },
                            end_point: Point {
                                row: 7,
                                column: 9,
                            },
                        },
                        kind: TypeDefinition(
                            TypeDefinition {
                                name: Name(
                                    "Baz",
                                ),
                            },
                        ),
                    },
                    Item {
                        range: Range {
                            start_byte: 111,
                            end_byte: 132,
                            start_point: Point {
                                row: 9,
                                column: 8,
                            },
                            end_point: Point {
                                row: 9,
                                column: 29,
                            },
                        },
                        kind: DirectiveDefinition(
                            DirectiveDefinition {
                                name: Name(
                                    "d",
                                ),
                            },
                        ),
                    },
                ],
            },
        }
    "#]]
    .assert_debug_eq(&items);
}
