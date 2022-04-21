use std::collections::HashSet;
use std::path::Path;

use crate::{DefDatabase, DefDatabaseStorage, ItemRes, Name};
use expect_test::expect;
use gqls_base_db::SourceDatabaseStorage;
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

pub(crate) use idx;

macro_rules! setup {
    ($db:ident: {
        $($file:ident: $text:expr,)*
     }) => {
        use gqls_base_db::{FileData, SourceDatabase};
        $db.set_projects(std::sync::Arc::new(
            maplit::hashmap! { "default" => maplit::hashset! { $($file),* } },
        ));
        $(
            $db.set_file_data($file, FileData::new($text, gqls_parse::parse_fresh($text)));
        )*
    };
}

pub(crate) use setup;

#[test]
fn test_definitions() {
    let mut db = TestDB::default();
    let mut vfs = Vfs::default();

    let foo = vfs.intern("foo");
    let foogql = r#"
        type Foo @qux {
           bar: Bar @qux
        }

        type Foo {
            foo: Foo @d
        }

        type Bar {
            foo: Foo
        }

        extend type Bar {
            i: Int! @qux
        }

        directive @qux on FIELD_DEFINITION | OBJECT

        scalar S @qux

        union U @qux = Foo | Bar

        input I @qux {
            foo: Foo @qux
        }

        interface Iface @qux {
            foo: Foo @qux
        }
    "#;

    let bar = vfs.intern("bar");
    let bargql = r#"
        type Bar {
            baz: Baz
        }

        type Baz @foo {
            foo: Foo
        }

        directive @d on FIELD
    "#;

    setup!(db: {
        foo: foogql,
        bar: bargql,
    });

    let item_map = db.item_map(foo);
    assert_eq!(
        *item_map,
        hashmap! {
            Name::unranged("Foo") => smallvec![idx!(0), idx!(1)],
            Name::unranged("Bar") => smallvec![idx!(2), idx!(3)],
            Name::unranged("qux") => smallvec![idx!(4)],
            Name::unranged("S") => smallvec![idx!(5)],
            Name::unranged("U") => smallvec![idx!(6)],
            Name::unranged("I") => smallvec![idx!(7)],
            Name::unranged("Iface") => smallvec![idx!(8)],
        }
    );

    let resolutions = db.resolve_item(bar, Name::unranged("Foo"));
    assert_eq!(
        resolutions.as_slice(),
        [ItemRes { file: foo, idx: idx!(0) }, ItemRes { file: foo, idx: idx!(1) }]
    );

    let resolutions = db.resolve_item(foo, Name::unranged("Bar"));
    assert_eq!(
        resolutions.into_iter().collect::<HashSet<_>>(),
        hashset! {
            ItemRes { file: bar, idx: idx!(0) },
            ItemRes { file: foo, idx: idx!(2) },
            ItemRes { file: foo, idx: idx!(3) },
        }
    );

    let resolutions = db.resolve_item(bar, Name::unranged("d"));
    assert_eq!(resolutions.as_slice(), [ItemRes { file: Path::new("bar"), idx: idx!(2) },]);

    let items = db.items(foo);
    expect![[r#"
        Items {
            items: Arena {
                len: 9,
                data: [
                    Item {
                        name: Foo,
                        range: Range {
                            start_byte: 9,
                            end_byte: 59,
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
                            Idx::<TypeDefinition>(0),
                        ),
                    },
                    Item {
                        name: Foo,
                        range: Range {
                            start_byte: 69,
                            end_byte: 113,
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
                            Idx::<TypeDefinition>(1),
                        ),
                    },
                    Item {
                        name: Bar,
                        range: Range {
                            start_byte: 123,
                            end_byte: 164,
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
                            Idx::<TypeDefinition>(2),
                        ),
                    },
                    Item {
                        name: Bar,
                        range: Range {
                            start_byte: 174,
                            end_byte: 226,
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
                            Idx::<TypeExtension>(0),
                        ),
                    },
                    Item {
                        name: qux,
                        range: Range {
                            start_byte: 236,
                            end_byte: 279,
                            start_point: Point {
                                row: 17,
                                column: 8,
                            },
                            end_point: Point {
                                row: 17,
                                column: 51,
                            },
                        },
                        kind: DirectiveDefinition(
                            Idx::<DirectiveDefinition>(0),
                        ),
                    },
                    Item {
                        name: S,
                        range: Range {
                            start_byte: 289,
                            end_byte: 302,
                            start_point: Point {
                                row: 19,
                                column: 8,
                            },
                            end_point: Point {
                                row: 19,
                                column: 21,
                            },
                        },
                        kind: TypeDefinition(
                            Idx::<TypeDefinition>(3),
                        ),
                    },
                    Item {
                        name: U,
                        range: Range {
                            start_byte: 312,
                            end_byte: 336,
                            start_point: Point {
                                row: 21,
                                column: 8,
                            },
                            end_point: Point {
                                row: 21,
                                column: 32,
                            },
                        },
                        kind: TypeDefinition(
                            Idx::<TypeDefinition>(4),
                        ),
                    },
                    Item {
                        name: I,
                        range: Range {
                            start_byte: 346,
                            end_byte: 396,
                            start_point: Point {
                                row: 23,
                                column: 8,
                            },
                            end_point: Point {
                                row: 25,
                                column: 9,
                            },
                        },
                        kind: TypeDefinition(
                            Idx::<TypeDefinition>(5),
                        ),
                    },
                    Item {
                        name: Iface,
                        range: Range {
                            start_byte: 406,
                            end_byte: 464,
                            start_point: Point {
                                row: 27,
                                column: 8,
                            },
                            end_point: Point {
                                row: 29,
                                column: 9,
                            },
                        },
                        kind: TypeDefinition(
                            Idx::<TypeDefinition>(6),
                        ),
                    },
                ],
            },
            types: Arena {
                len: 7,
                data: [
                    TypeDefinition {
                        directives: [
                            Directive {
                                range: Range {
                                    start_byte: 18,
                                    end_byte: 22,
                                    start_point: Point {
                                        row: 1,
                                        column: 17,
                                    },
                                    end_point: Point {
                                        row: 1,
                                        column: 21,
                                    },
                                },
                                name: qux,
                            },
                        ],
                    },
                    TypeDefinition {
                        directives: [],
                    },
                    TypeDefinition {
                        directives: [],
                    },
                    TypeDefinition {
                        directives: [
                            Directive {
                                range: Range {
                                    start_byte: 298,
                                    end_byte: 302,
                                    start_point: Point {
                                        row: 19,
                                        column: 17,
                                    },
                                    end_point: Point {
                                        row: 19,
                                        column: 21,
                                    },
                                },
                                name: qux,
                            },
                        ],
                    },
                    TypeDefinition {
                        directives: [
                            Directive {
                                range: Range {
                                    start_byte: 320,
                                    end_byte: 324,
                                    start_point: Point {
                                        row: 21,
                                        column: 16,
                                    },
                                    end_point: Point {
                                        row: 21,
                                        column: 20,
                                    },
                                },
                                name: qux,
                            },
                        ],
                    },
                    TypeDefinition {
                        directives: [
                            Directive {
                                range: Range {
                                    start_byte: 354,
                                    end_byte: 358,
                                    start_point: Point {
                                        row: 23,
                                        column: 16,
                                    },
                                    end_point: Point {
                                        row: 23,
                                        column: 20,
                                    },
                                },
                                name: qux,
                            },
                        ],
                    },
                    TypeDefinition {
                        directives: [
                            Directive {
                                range: Range {
                                    start_byte: 422,
                                    end_byte: 426,
                                    start_point: Point {
                                        row: 27,
                                        column: 24,
                                    },
                                    end_point: Point {
                                        row: 27,
                                        column: 28,
                                    },
                                },
                                name: qux,
                            },
                        ],
                    },
                ],
            },
            directives: Arena {
                len: 1,
                data: [
                    DirectiveDefinition,
                ],
            },
            type_exts: Arena {
                len: 1,
                data: [
                    TypeExtension {
                        directives: [],
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
                        name: Bar,
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
                            Idx::<TypeDefinition>(0),
                        ),
                    },
                    Item {
                        name: Baz,
                        range: Range {
                            start_byte: 60,
                            end_byte: 106,
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
                            Idx::<TypeDefinition>(1),
                        ),
                    },
                    Item {
                        name: d,
                        range: Range {
                            start_byte: 116,
                            end_byte: 137,
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
                            Idx::<DirectiveDefinition>(0),
                        ),
                    },
                ],
            },
            types: Arena {
                len: 2,
                data: [
                    TypeDefinition {
                        directives: [],
                    },
                    TypeDefinition {
                        directives: [
                            Directive {
                                range: Range {
                                    start_byte: 69,
                                    end_byte: 73,
                                    start_point: Point {
                                        row: 5,
                                        column: 17,
                                    },
                                    end_point: Point {
                                        row: 5,
                                        column: 21,
                                    },
                                },
                                name: foo,
                            },
                        ],
                    },
                ],
            },
            directives: Arena {
                len: 1,
                data: [
                    DirectiveDefinition,
                ],
            },
            type_exts: Arena {
                len: 0,
                data: [],
            },
        }
    "#]]
    .assert_debug_eq(&items);
}
